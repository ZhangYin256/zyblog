use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use std::sync::Arc;
use tower::util::ServiceExt;
use zyblog::config::Config;
use zyblog::handlers::pulls::{AddCommentRequest, CreatePullRequest, UpdatePullRequest};
use zyblog::routes::pulls::{pull_item_routes, pulls_routes};
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
            backup_dir: "./test_backups".to_string(),
            backup_interval_hours: 24,
            backup_retention_count: 10,
        },
    })
}

/// Build a test router for post-level pulls routes (create & list).
fn pulls_app() -> Router {
    let state = test_state();
    pulls_routes().with_state(state)
}

/// Build a test router for pull-item routes (update & comment).
fn pull_item_app() -> Router {
    let state = test_state();
    pull_item_routes().with_state(state)
}

// ===== Request type deserialization tests =====

#[test]
fn create_pull_request_deserialize() {
    let json = r#"{
        "user_email": "contributor@example.com",
        "content": "Fix typo in README"
    }"#;

    let req: CreatePullRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.user_email, "contributor@example.com");
    assert_eq!(req.content, "Fix typo in README");
}

#[test]
fn update_pull_request_deserialize_open() {
    let json = r#"{"status": "open"}"#;
    let req: UpdatePullRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.status, "open");
}

#[test]
fn update_pull_request_deserialize_closed() {
    let json = r#"{"status": "closed"}"#;
    let req: UpdatePullRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.status, "closed");
}

#[test]
fn update_pull_request_deserialize_merged() {
    let json = r#"{"status": "merged"}"#;
    let req: UpdatePullRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.status, "merged");
}

#[test]
fn add_comment_request_deserialize() {
    let json = r#"{
        "user_email": "reviewer@example.com",
        "content": "Looks good to me!"
    }"#;

    let req: AddCommentRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.user_email, "reviewer@example.com");
    assert_eq!(req.content, "Looks good to me!");
}

// ===== Routing tests (no DB → 500) =====

#[tokio::test]
async fn list_pulls_returns_error_without_db() {
    let app = pulls_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn list_pulls_returns_error_status_without_db() {
    let app = pulls_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn create_pull_returns_error_without_db() {
    let app = pulls_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"user_email":"test@test.com","content":"fix something"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn update_pull_returns_error_without_db() {
    let app = pull_item_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"status":"closed"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn add_comment_returns_error_without_db() {
    let app = pull_item_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/comments")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"user_email":"test@test.com","content":"nice work"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
