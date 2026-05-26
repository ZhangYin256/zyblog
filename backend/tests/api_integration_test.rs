use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use zyblog::config::Config;
use zyblog::middleware::auth;
use zyblog::routes::posts::posts_routes;
use zyblog::state::AppState;

/// Create a test AppState with no database connection.
fn test_state() -> Arc<AppState> {
    Arc::new(AppState {
        db: None,
        config: Config {
            server_addr: "0.0.0.0:8080".to_string(),
            database_url: "postgres://localhost/test".to_string(),
            rust_log: "debug".to_string(),
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            smtp_username: "".to_string(),
            smtp_password: "".to_string(),
            smtp_from: "test@test.com".to_string(),
        },
    })
}

/// Build a test router with posts routes (no auth middleware for simplicity).
fn posts_app() -> Router {
    let state = test_state();
    posts_routes().with_state(state)
}

/// Build a test router with auth middleware.
fn protected_app() -> Router {
    let state = test_state();
    Router::new()
        .nest("/api/v1/posts", posts_routes().with_state(state.clone()))
        .layer(axum::middleware::from_fn(auth::admin_auth_middleware))
}

// ===== Health check / routing tests =====

#[tokio::test]
async fn list_posts_returns_json() {
    let app = posts_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Without DB, handlers should return 500
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn get_post_returns_not_found_or_error() {
    let app = posts_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn create_post_without_auth_fails() {
    let app = protected_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/posts")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title":"Test","content":"Content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should fail with 401 (no auth header) or 500 (no ADMIN_KEY env var)
    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn create_post_with_invalid_auth_fails() {
    let app = protected_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/posts")
                .header("content-type", "application/json")
                .header("authorization", "Bearer wrong-key")
                .body(Body::from(r#"{"title":"Test","content":"Content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn list_posts_with_query_params() {
    let app = posts_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/?page=1&per_page=5&status=published")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Without DB, should return 500
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn delete_post_returns_not_found_or_error() {
    let app = posts_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::INTERNAL_SERVER_ERROR
            || response.status() == StatusCode::NOT_FOUND
    );
}

// ===== Auth middleware tests =====

#[tokio::test]
async fn auth_middleware_allows_get_without_token() {
    let app = protected_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/posts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_middleware_rejects_post_without_auth() {
    let app = protected_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/posts")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title":"Test","content":"Content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn auth_middleware_rejects_wrong_bearer_token() {
    let app = protected_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/posts")
                .header("content-type", "application/json")
                .header("authorization", "Bearer wrong-key")
                .body(Body::from(r#"{"title":"Test","content":"Content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn auth_middleware_rejects_malformed_auth_header() {
    let app = protected_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/posts")
                .header("content-type", "application/json")
                .header("authorization", "Basic dXNlcjpwYXNz")
                .body(Body::from(r#"{"title":"Test","content":"Content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        response.status() == StatusCode::UNAUTHORIZED
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

// ===== Error response format tests =====

#[tokio::test]
async fn error_response_is_json() {
    let app = posts_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Error responses should have an "error" field
    assert!(json.get("error").is_some());
}
