use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use super::jwt::{role_rank, Claims};

pub async fn require_role_middleware(
    min_role: &str,
    request: Request<Body>,
    next: Next,
) -> Response {
    let min_rank = role_rank(min_role);

    let claims = match request.extensions().get::<Claims>() {
        Some(claims) => claims.clone(),
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Authentication required" })),
            )
                .into_response();
        }
    };

    let user_rank = role_rank(&claims.role);
    if user_rank < min_rank {
        tracing::warn!(
            "Role check failed: user_role='{}' ({}), required='{}' ({})",
            claims.role,
            user_rank,
            min_role,
            min_rank
        );
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Insufficient permissions" })),
        )
            .into_response();
    }

    next.run(request).await
}
