use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::middleware::jwt::Claims;
use crate::state::AppState;

/// 判断请求路径是否为公开的 POST 端点（不需要管理员认证）
///
/// 公开端点包括：
/// - `POST /api/v1/posts/:id/pulls` — 创建 PR（读者可提交修改建议）
/// - `POST /api/v1/pulls/:id/comments` — 添加评论（读者可评论 PR）
/// - `POST /api/v1/subscribers` — 订阅（任何人可订阅）
/// - `POST /api/v1/auth/*` — 认证端点（登录、注册、刷新）
fn is_public_post_endpoint(path: &str) -> bool {
    // POST /api/v1/posts/:id/pulls
    if path.starts_with("/api/v1/posts/") && path.ends_with("/pulls") {
        let middle = &path["/api/v1/posts/".len()..path.len() - "/pulls".len()];
        // 确保中间部分是数字 ID，且 /pulls 前有 /
        if middle.ends_with('/') && middle[..middle.len() - 1].chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    // POST /api/v1/pulls/:id/comments
    if path.starts_with("/api/v1/pulls/") && path.ends_with("/comments") {
        let middle = &path["/api/v1/pulls/".len()..path.len() - "/comments".len()];
        if middle.ends_with('/') && middle[..middle.len() - 1].chars().all(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    // POST /api/v1/subscribers
    if path == "/api/v1/subscribers" {
        return true;
    }
    // POST /api/v1/auth/* — authentication endpoints (login, register, refresh)
    if path.starts_with("/api/v1/auth/") {
        return true;
    }
    false
}

/// 管理员认证中间件
///
/// 通过要求 `Authorization` 头中的有效 Bearer 令牌来保护写操作
/// （POST、PUT、DELETE、PATCH）。令牌与 `ADMIN_KEY` 环境变量进行比较。
///
/// 读操作（GET、HEAD、OPTIONS）无需认证即可通过。
/// 部分 POST 端点（PR 创建、评论、订阅）为公开端点，无需认证。
pub async fn admin_auth_middleware(
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

    if method == axum::http::Method::POST {
        let path = request.uri().path();
        if is_public_post_endpoint(path) {
            return next.run(request).await;
        }
    }

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

    // Try JWT validation first
    if let Ok(claims) = crate::middleware::jwt::validate_jwt(token, &state.config.jwt_secret) {
        tracing::debug!("JWT auth success: user_id={}, role={}", claims.sub, claims.role);
        request.extensions_mut().insert(claims);
        return next.run(request).await;
    }

    // Fall back to ADMIN_KEY
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

    if token == admin_key {
        let admin_claims = Claims {
            sub: 0,
            role: "admin".to_string(),
            exp: 0,
        };
        request.extensions_mut().insert(admin_claims);
        return next.run(request).await;
    }

    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "Invalid token" })),
    )
        .into_response()
}
