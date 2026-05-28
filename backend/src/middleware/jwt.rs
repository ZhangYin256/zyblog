use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub role: String,
    pub exp: usize,
}

const ROLE_HIERARCHY: &[(&str, u32)] = &[
    ("visitor", 0),
    ("contributor", 1),
    ("admin", 2),
];

pub fn role_rank(role: &str) -> u32 {
    ROLE_HIERARCHY
        .iter()
        .find(|(r, _)| *r == role)
        .map(|(_, rank)| *rank)
        .unwrap_or(0)
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

fn extract_bearer_token(request: &Request<Body>) -> Option<String> {
    request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(|token| token.trim().to_string())
}

pub async fn dual_auth_middleware(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();

    if method == axum::http::Method::GET
        || method == axum::http::Method::HEAD
        || method == axum::http::Method::OPTIONS
    {
        return next.run(request).await;
    }

    if let Some(token) = extract_bearer_token(&request) {
        if let Ok(claims) = validate_jwt(&token, &state.config.jwt_secret) {
            tracing::debug!("JWT auth success: user_id={}, role={}", claims.sub, claims.role);
            request.extensions_mut().insert(claims);
            return next.run(request).await;
        }

        if !state.config.admin_key.is_empty() && token == state.config.admin_key {
            tracing::debug!("ADMIN_KEY auth success");
            let admin_claims = Claims {
                sub: 0,
                role: "admin".to_string(),
                exp: 0,
            };
            request.extensions_mut().insert(admin_claims);
            return next.run(request).await;
        }
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Unauthorized" })),
    )
        .into_response()
}
