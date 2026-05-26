use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Admin authentication middleware.
///
/// Protects write operations (POST, PUT, DELETE, PATCH) by requiring a valid
/// Bearer token in the `Authorization` header. The token is compared against
/// the `ADMIN_KEY` environment variable.
///
/// Read operations (GET, HEAD, OPTIONS) pass through without authentication.
pub async fn admin_auth_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();

    // Read-only methods don't need auth
    if method == axum::http::Method::GET
        || method == axum::http::Method::HEAD
        || method == axum::http::Method::OPTIONS
    {
        return next.run(request).await;
    }

    // Read ADMIN_KEY from environment
    let admin_key = match std::env::var("ADMIN_KEY") {
        Ok(key) => key,
        Err(_) => {
            tracing::error!("ADMIN_KEY environment variable not set");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Server configuration error" })),
            )
                .into_response();
        }
    };

    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) => {
            if let Some(token) = header.strip_prefix("Bearer ") {
                token.trim()
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Invalid authorization header format" })),
                )
                    .into_response();
            }
        }
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Missing authorization header" })),
            )
                .into_response();
        }
    };

    // Validate token
    if token != admin_key {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response();
    }

    next.run(request).await
}
