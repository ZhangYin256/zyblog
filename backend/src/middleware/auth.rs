use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// 管理员认证中间件
///
/// 通过要求 `Authorization` 头中的有效 Bearer 令牌来保护写操作
/// （POST、PUT、DELETE、PATCH）。令牌与 `ADMIN_KEY` 环境变量进行比较。
///
/// 读操作（GET、HEAD、OPTIONS）无需认证即可通过
pub async fn admin_auth_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().clone();

    // 只读方法不需要认证
    if method == axum::http::Method::GET
        || method == axum::http::Method::HEAD
        || method == axum::http::Method::OPTIONS
    {
        return next.run(request).await;
    }

    // 从环境变量读取 ADMIN_KEY
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

    // 提取 Authorization 头
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

    // 验证令牌
    if token != admin_key {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid token" })),
        )
            .into_response();
    }

    next.run(request).await
}
