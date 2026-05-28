use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
    Json,
};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::middleware::jwt::Claims;
use crate::models::{refresh_token, user};
use crate::state::AppState;

/// Register request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

/// Login request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Refresh token request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// User response (public profile)
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub role: String,
    pub avatar_url: Option<String>,
}

/// Auth response with tokens
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

/// Refresh response with new access token
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
}

/// Single user response (for /me endpoint)
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MeResponse {
    pub user: UserResponse,
}

/// Update profile request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateProfileRequest {
    pub email: Option<String>,
    pub name: Option<String>,
}

/// Change password request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

/// Generic success response
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SuccessResponse {
    pub message: String,
}

fn user_to_response(user: &user::Model) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email.clone(),
        name: user.name.clone(),
        role: user.role.clone(),
        avatar_url: user.avatar_url.clone(),
    }
}

fn generate_access_token(user_id: i32, role: &str, secret: &str, expiry_secs: u64) -> String {
    let claims = Claims {
        sub: user_id,
        role: role.to_string(),
        exp: (Utc::now() + Duration::seconds(expiry_secs as i64)).timestamp() as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

fn generate_refresh_token() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

/// POST /api/v1/auth/register - Register a new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Bad request"),
        (status = 409, description = "Email already exists")
    ),
    tag = "auth"
)]
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // Validate email format (basic check)
    if !body.email.contains('@') || body.email.trim().is_empty() {
        return Err(AppError::BadRequest("Invalid email format".to_string()));
    }

    // Validate password length
    if body.password.len() < 6 {
        return Err(AppError::BadRequest(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    // Validate name not empty
    if body.name.trim().is_empty() {
        return Err(AppError::BadRequest("Name is required".to_string()));
    }

    // Check if email already exists
    let existing = user::Entity::find()
        .filter(user::Column::Email.eq(&body.email))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Hash password
    let password_hash = bcrypt::hash(&body.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash password: {}", e)))?;

    // Create user
    let now = Utc::now();
    let new_user = user::ActiveModel {
        email: Set(body.email.clone()),
        password_hash: Set(Some(password_hash)),
        name: Set(body.name.clone()),
        role: Set("visitor".to_string()),
        avatar_url: Set(None),
        github_id: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let user = new_user.insert(db).await?;

    // Generate tokens
    let access_token = generate_access_token(
        user.id,
        &user.role,
        &state.config.jwt_secret,
        state.config.jwt_access_expiry,
    );
    let refresh_token_str = generate_refresh_token();

    // Store refresh token hash
    let refresh_token_hash = bcrypt::hash(&refresh_token_str, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash refresh token: {}", e)))?;

    let new_refresh_token = refresh_token::ActiveModel {
        user_id: Set(user.id),
        token_hash: Set(refresh_token_hash),
        expires_at: Set(now + Duration::seconds(state.config.jwt_refresh_expiry as i64)),
        created_at: Set(now),
        ..Default::default()
    };

    new_refresh_token.insert(db).await?;

    let response = AuthResponse {
        user: user_to_response(&user),
        access_token,
        refresh_token: refresh_token_str,
    };

    Ok((StatusCode::CREATED, Json(response)))
}

/// POST /api/v1/auth/login - Login with email and password
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // Find user by email
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&body.email))
        .one(db)
        .await?
        .ok_or_else(|| AppError::BadRequest("Invalid email or password".to_string()))?;

    // Verify password
    let password_hash = user
        .password_hash
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Invalid email or password".to_string()))?;

    let is_valid = bcrypt::verify(&body.password, password_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to verify password: {}", e)))?;

    if !is_valid {
        return Err(AppError::BadRequest("Invalid email or password".to_string()));
    }

    // Generate tokens
    let access_token = generate_access_token(
        user.id,
        &user.role,
        &state.config.jwt_secret,
        state.config.jwt_access_expiry,
    );
    let refresh_token_str = generate_refresh_token();

    // Store refresh token hash
    let now = Utc::now();
    let refresh_token_hash = bcrypt::hash(&refresh_token_str, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash refresh token: {}", e)))?;

    let new_refresh_token = refresh_token::ActiveModel {
        user_id: Set(user.id),
        token_hash: Set(refresh_token_hash),
        expires_at: Set(now + Duration::seconds(state.config.jwt_refresh_expiry as i64)),
        created_at: Set(now),
        ..Default::default()
    };

    new_refresh_token.insert(db).await?;

    let response = AuthResponse {
        user: user_to_response(&user),
        access_token,
        refresh_token: refresh_token_str,
    };

    Ok(Json(response))
}

/// POST /api/v1/auth/refresh - Refresh access token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed", body = RefreshResponse),
        (status = 401, description = "Invalid refresh token")
    ),
    tag = "auth"
)]
pub async fn refresh(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // Find all refresh tokens and check against the provided token
    // We need to check each one since we store hashes
    let refresh_tokens = refresh_token::Entity::find()
        .all(db)
        .await?;

    let now = Utc::now();
    let mut matched_token = None;
    let mut matched_user_id = None;

    for rt in refresh_tokens {
        // Skip expired tokens
        if rt.expires_at < now {
            continue;
        }

        // Verify the refresh token against the hash
        let is_valid = bcrypt::verify(&body.refresh_token, &rt.token_hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to verify refresh token: {}", e)))?;

        if is_valid {
            matched_token = Some(rt.clone());
            matched_user_id = Some(rt.user_id);
            break;
        }
    }

    let (token_record, user_id) = matched_token
        .zip(matched_user_id)
        .ok_or_else(|| AppError::BadRequest("Invalid or expired refresh token".to_string()))?;

    // Delete the used refresh token
    let token_model: refresh_token::ActiveModel = token_record.into();
    refresh_token::Entity::delete_by_id(token_model.id.unwrap())
        .exec(db)
        .await?;

    // Find the user
    let user = user::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Generate new access token
    let access_token = generate_access_token(
        user.id,
        &user.role,
        &state.config.jwt_secret,
        state.config.jwt_access_expiry,
    );

    Ok(Json(RefreshResponse { access_token }))
}

/// GET /api/v1/auth/me - Get current user profile
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    responses(
        (status = 200, description = "Current user profile", body = MeResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "auth"
)]
pub async fn me(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<MeResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // Extract authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) => {
            if let Some(token) = header.strip_prefix("Bearer ") {
                token.trim()
            } else {
                return Err(AppError::BadRequest(
                    "Invalid authorization header format".to_string(),
                ));
            }
        }
        None => {
            return Err(AppError::BadRequest(
                "Missing authorization header".to_string(),
            ));
        }
    };

    // Try JWT validation first
    let claims = match crate::middleware::jwt::validate_jwt(token, &state.config.jwt_secret) {
        Ok(claims) => claims,
        Err(_) => {
            // If JWT fails, check ADMIN_KEY
            let admin_key = std::env::var("ADMIN_KEY").map_err(|_| {
                AppError::Internal(anyhow::anyhow!("ADMIN_KEY not set"))
            })?;

            if token == admin_key {
                // Return admin user info
                // Try to find an admin user, or return a synthetic admin response
                let admin_user = user::Entity::find()
                    .filter(user::Column::Role.eq("admin"))
                    .one(db)
                    .await?;

                if let Some(admin) = admin_user {
                    return Ok(Json(MeResponse {
                        user: user_to_response(&admin),
                    }));
                } else {
                    // No admin user in DB, return synthetic response
                    return Ok(Json(MeResponse {
                        user: UserResponse {
                            id: 0,
                            email: "admin@zyblog.local".to_string(),
                            name: "Admin".to_string(),
                            role: "admin".to_string(),
                            avatar_url: None,
                        },
                    }));
                }
            } else {
                return Err(AppError::BadRequest("Invalid token".to_string()));
            }
        }
    };

    // Find user from JWT claims
    let user = user::Entity::find_by_id(claims.sub)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(MeResponse {
        user: user_to_response(&user),
    }))
}

/// PUT /api/v1/auth/profile - Update current user's email and name
#[utoipa::path(
    put,
    path = "/api/v1/auth/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated", body = MeResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Email already taken")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "auth"
)]
pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<Claims>,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<MeResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let user = user::Entity::find_by_id(claims.sub)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let mut active: user::ActiveModel = user.into();

    if let Some(email) = &body.email {
        if !email.contains('@') || email.trim().is_empty() {
            return Err(AppError::BadRequest("Invalid email format".to_string()));
        }
        let existing = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .filter(user::Column::Id.ne(claims.sub))
            .one(db)
            .await?;
        if existing.is_some() {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }
        active.email = Set(email.clone());
    }

    if let Some(name) = &body.name {
        if name.trim().is_empty() {
            return Err(AppError::BadRequest("Name cannot be empty".to_string()));
        }
        active.name = Set(name.clone());
    }

    active.updated_at = Set(Utc::now());

    let updated = active.update(db).await?;

    Ok(Json(MeResponse {
        user: user_to_response(&updated),
    }))
}

/// PUT /api/v1/auth/password - Change current user's password
#[utoipa::path(
    put,
    path = "/api/v1/auth/password",
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "Password changed", body = SuccessResponse),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "auth"
)]
pub async fn change_password(
    State(state): State<Arc<AppState>>,
    axum::Extension(claims): axum::Extension<Claims>,
    Json(body): Json<ChangePasswordRequest>,
) -> Result<Json<SuccessResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    if body.new_password.len() < 6 {
        return Err(AppError::BadRequest(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    let user = user::Entity::find_by_id(claims.sub)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let password_hash = user
        .password_hash
        .as_ref()
        .ok_or_else(|| AppError::BadRequest("Account has no password set".to_string()))?;

    let is_valid = bcrypt::verify(&body.current_password, password_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to verify password: {}", e)))?;

    if !is_valid {
        return Err(AppError::BadRequest("Current password is incorrect".to_string()));
    }

    let new_hash = bcrypt::hash(&body.new_password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash password: {}", e)))?;

    let mut active: user::ActiveModel = user.into();
    active.password_hash = Set(Some(new_hash));
    active.updated_at = Set(Utc::now());
    active.update(db).await?;

    Ok(Json(SuccessResponse {
        message: "Password changed successfully".to_string(),
    }))
}

// ---------------------------------------------------------------------------
// GitHub OAuth
// ---------------------------------------------------------------------------

/// Response returned by the JSON-based GitHub callback endpoint.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
}

/// Query parameters returned by GitHub after the user authorizes the app.
#[derive(Deserialize)]
pub struct GitHubCallbackQuery {
    pub code: Option<String>,
    #[allow(dead_code)]
    pub state: Option<String>,
}

#[derive(Deserialize)]
struct GitHubTokenResponse {
    access_token: Option<String>,
    #[allow(dead_code)]
    token_type: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Deserialize)]
struct GitHubUser {
    id: u64,
    login: String,
    email: Option<String>,
    name: Option<String>,
    avatar_url: Option<String>,
}

/// Exchange a GitHub OAuth authorization `code` for an access token.
async fn exchange_code_for_token(
    code: &str,
    config: &crate::config::Config,
) -> Result<String, AppError> {
    let client = reqwest::Client::new();

    let params = serde_json::json!({
        "client_id": config.github_client_id,
        "client_secret": config.github_client_secret,
        "code": code,
    });

    let resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .json(&params)
        .send()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to contact GitHub: {e}")))?;

    let token_resp: GitHubTokenResponse = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to parse GitHub token response: {e}")))?;

    if let Some(err) = token_resp.error {
        let desc = token_resp.error_description.unwrap_or_default();
        tracing::error!("GitHub token exchange error: {err} - {desc}");
        return Err(AppError::BadRequest(format!("GitHub OAuth error: {err}")));
    }

    token_resp
        .access_token
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("GitHub did not return an access token")))
}

/// Fetch the authenticated user's profile from GitHub.
async fn fetch_github_user(access_token: &str) -> Result<GitHubUser, AppError> {
    let client = reqwest::Client::new();

    let resp = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("User-Agent", "zyblog")
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch GitHub user: {e}")))?;

    let user_info: GitHubUser = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to parse GitHub user: {e}")))?;

    Ok(user_info)
}

/// Fetch the primary verified email from `/user/emails` as a fallback
/// when the user's profile email is not publicly visible.
async fn fetch_github_primary_email(access_token: &str) -> Result<Option<String>, AppError> {
    let client = reqwest::Client::new();

    let resp = client
        .get("https://api.github.com/user/emails")
        .header("Authorization", format!("Bearer {access_token}"))
        .header("User-Agent", "zyblog")
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to fetch GitHub emails: {e}")))?;

    #[derive(Deserialize)]
    struct EmailEntry {
        email: String,
        primary: bool,
        verified: bool,
    }

    let emails: Vec<EmailEntry> = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to parse GitHub emails: {e}")))?;

    Ok(emails
        .into_iter()
        .find(|e| e.primary && e.verified)
        .map(|e| e.email))
}

/// Account linking logic:
/// 1. Match by github_id → login
/// 2. Match by email → link GitHub to existing account
/// 3. No match → create new visitor user
async fn find_or_create_user(
    db: &sea_orm::DatabaseConnection,
    gh_user: &GitHubUser,
    github_id: &str,
    email: Option<&str>,
) -> Result<user::Model, AppError> {
    // 1. Match by github_id
    if let Some(existing) = user::Entity::find()
        .filter(user::Column::GithubId.eq(github_id))
        .one(db)
        .await?
    {
        tracing::info!("Found existing user by github_id: {}", existing.id);
        return Ok(existing);
    }

    // 2. Match by email (if available)
    if let Some(email) = email {
        if let Some(existing) = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await?
        {
            tracing::info!(
                "Linking GitHub account to existing user by email: {}",
                existing.id
            );
            let mut active: user::ActiveModel = existing.into();
            active.github_id = Set(Some(github_id.to_string()));
            let updated = active.update(db).await?;
            return Ok(updated);
        }
    }

    // 3. Create new user
    let email = email
        .map(|e| e.to_string())
        .unwrap_or_else(|| format!("{}@users.noreply.github.com", gh_user.login));

    let name = gh_user
        .name
        .clone()
        .unwrap_or_else(|| gh_user.login.clone());

    tracing::info!("Creating new user from GitHub: login={}", gh_user.login);

    let now: DateTime<Utc> = Utc::now();
    let new_user = user::ActiveModel {
        id: Default::default(),
        email: Set(email),
        password_hash: Set(None),
        name: Set(name),
        avatar_url: Set(gh_user.avatar_url.clone()),
        role: Set("visitor".to_string()),
        github_id: Set(Some(github_id.to_string())),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let user = new_user.insert(db).await?;
    Ok(user)
}

/// GET /api/v1/auth/github
///
/// Redirect the browser to GitHub's OAuth authorization page.
/// Includes a random `state` parameter for CSRF protection.
#[utoipa::path(
    get,
    path = "/api/v1/auth/github",
    responses(
        (status = 302, description = "Redirect to GitHub OAuth")
    ),
    tag = "auth"
)]
pub async fn github_login(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    let config = &state.config;

    if config.github_client_id.is_empty() {
        return Err(AppError::BadRequest(
            "GitHub OAuth is not configured".to_string(),
        ));
    }

    let csrf_state = uuid::Uuid::new_v4().to_string();

    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email&state={}",
        config.github_client_id,
        urlencoding::encode(&config.github_redirect_uri),
        csrf_state,
    );

    tracing::info!("Redirecting to GitHub OAuth: state={}", csrf_state);

    Ok(Redirect::temporary(&url))
}

/// GET /api/v1/auth/github/url
///
/// Returns the GitHub OAuth authorization URL as JSON.
/// Used by the frontend to check availability and get the URL
/// before navigating. Returns 400 if GitHub OAuth is not configured.
#[utoipa::path(
    get,
    path = "/api/v1/auth/github/url",
    responses(
        (status = 200, description = "GitHub OAuth URL", body = String),
        (status = 400, description = "GitHub OAuth not configured")
    ),
    tag = "auth"
)]
pub async fn github_login_url(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    let config = &state.config;

    if config.github_client_id.is_empty() {
        return Err(AppError::BadRequest(
            "GitHub OAuth is not configured".to_string(),
        ));
    }

    let csrf_state = uuid::Uuid::new_v4().to_string();

    let url = format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email&state={}",
        config.github_client_id,
        urlencoding::encode(&config.github_redirect_uri),
        csrf_state,
    );

    tracing::info!("Returning GitHub OAuth URL: state={}", csrf_state);

    Ok(Json(serde_json::json!({ "url": url })))
}

/// GET /api/v1/auth/github/callback
///
/// Handle the GitHub OAuth callback. Exchanges the authorization `code`
/// for an access token, fetches the GitHub user profile, performs account
/// linking / creation, then redirects to the frontend with JWT tokens in
/// the URL fragment (hash) so the SPA can capture them.
#[utoipa::path(
    get,
    path = "/api/v1/auth/github/callback",
    params(
        ("code" = Option<String>, Query, description = "GitHub authorization code"),
        ("state" = Option<String>, Query, description = "CSRF state token"),
    ),
    responses(
        (status = 302, description = "Redirect to frontend with tokens")
    ),
    tag = "auth"
)]
pub async fn github_callback(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GitHubCallbackQuery>,
) -> Result<impl IntoResponse, AppError> {
    let config = &state.config;
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let code = query.code.ok_or_else(|| {
        AppError::BadRequest("Missing 'code' parameter".to_string())
    })?;

    tracing::info!("GitHub callback received, exchanging code...");

    let gh_access_token = exchange_code_for_token(&code, config).await?;

    let gh_user = fetch_github_user(&gh_access_token).await?;
    let github_id = gh_user.id.to_string();

    let email = match gh_user.email.as_ref().filter(|e| !e.is_empty()) {
        Some(e) => Some(e.clone()),
        None => fetch_github_primary_email(&gh_access_token)
            .await
            .ok()
            .flatten(),
    };

    tracing::info!(
        "GitHub user: id={}, login={}, email={:?}",
        github_id,
        gh_user.login,
        email
    );

    let user = find_or_create_user(db, &gh_user, &github_id, email.as_deref()).await?;

    tracing::info!("User authenticated: id={}, role={}", user.id, user.role);

    let access_token = generate_access_token(
        user.id,
        &user.role,
        &config.jwt_secret,
        config.jwt_access_expiry,
    );
    let refresh_token_str = generate_refresh_token();

    let now = Utc::now();
    let refresh_token_hash = bcrypt::hash(&refresh_token_str, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash refresh token: {}", e)))?;

    let new_refresh_token = refresh_token::ActiveModel {
        user_id: Set(user.id),
        token_hash: Set(refresh_token_hash),
        expires_at: Set(now + Duration::seconds(config.jwt_refresh_expiry as i64)),
        created_at: Set(now),
        ..Default::default()
    };

    new_refresh_token.insert(db).await?;

    let frontend_base = std::env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    let redirect_url = format!(
        "{}/auth/callback#access_token={}&refresh_token={}",
        frontend_base.trim_end_matches('/'),
        urlencoding::encode(&access_token),
        urlencoding::encode(&refresh_token_str),
    );

    Ok(Redirect::temporary(&redirect_url))
}

/// GET /api/v1/auth/github/token
///
/// Alternative to the redirect-based flow: returns tokens as JSON.
/// Useful for API clients or testing.
#[utoipa::path(
    get,
    path = "/api/v1/auth/github/token",
    params(
        ("code" = Option<String>, Query, description = "GitHub authorization code"),
    ),
    responses(
        (status = 200, description = "JWT tokens", body = TokenResponse),
        (status = 400, description = "Missing or invalid code"),
        (status = 500, description = "Database not available")
    ),
    tag = "auth"
)]
pub async fn github_callback_json(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GitHubCallbackQuery>,
) -> Result<Json<TokenResponse>, AppError> {
    let config = &state.config;
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let code = query.code.ok_or_else(|| {
        AppError::BadRequest("Missing 'code' parameter".to_string())
    })?;

    let gh_access_token = exchange_code_for_token(&code, config).await?;
    let gh_user = fetch_github_user(&gh_access_token).await?;
    let github_id = gh_user.id.to_string();

    let email = match gh_user.email.as_ref().filter(|e| !e.is_empty()) {
        Some(e) => Some(e.clone()),
        None => fetch_github_primary_email(&gh_access_token)
            .await
            .ok()
            .flatten(),
    };

    let user = find_or_create_user(db, &gh_user, &github_id, email.as_deref()).await?;

    let access_token = generate_access_token(
        user.id,
        &user.role,
        &config.jwt_secret,
        config.jwt_access_expiry,
    );
    let refresh_token_str = generate_refresh_token();

    let now = Utc::now();
    let refresh_token_hash = bcrypt::hash(&refresh_token_str, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to hash refresh token: {}", e)))?;

    let new_refresh_token = refresh_token::ActiveModel {
        user_id: Set(user.id),
        token_hash: Set(refresh_token_hash),
        expires_at: Set(now + Duration::seconds(config.jwt_refresh_expiry as i64)),
        created_at: Set(now),
        ..Default::default()
    };

    new_refresh_token.insert(db).await?;

    Ok(Json(TokenResponse {
        access_token,
        refresh_token: refresh_token_str,
        token_type: "Bearer".to_string(),
    }))
}
