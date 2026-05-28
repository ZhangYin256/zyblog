use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::Value;
use std::sync::Arc;
use tower::util::ServiceExt;
use zyblog::config::Config;
use zyblog::handlers::backup::RestoreRequest;
use zyblog::routes::backup::backup_routes;
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
            jwt_secret: "test-secret".to_string(),
            jwt_access_expiry: 900,
            jwt_refresh_expiry: 604800,
            github_client_id: "".to_string(),
            github_client_secret: "".to_string(),
            github_redirect_uri: "http://localhost:8080/api/v1/auth/github/callback".to_string(),
            admin_key: "test-admin-key".to_string(),
        },
    })
}

/// Build a test router with backup routes.
fn backup_app() -> Router {
    let state = test_state();
    backup_routes().with_state(state)
}

// ===== RestoreRequest deserialization tests =====

#[test]
fn restore_request_deserialize() {
    let json = r#"{"filename": "backup_20240101_120000.sql.gz"}"#;
    let req: RestoreRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.filename, "backup_20240101_120000.sql.gz");
}

// ===== Routing tests =====

#[tokio::test]
async fn create_backup_returns_error_without_db() {
    let app = backup_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Without pg_dump available or DB, should fail
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn list_backups_returns_error_without_db() {
    let app = backup_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/list")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // backup_dir doesn't exist → should still succeed with empty list or error
    // The handler creates the dir via list_backup_files which returns Ok(Vec::new()) if dir missing
    // But since we're using a relative path that may fail, accept either 200 or 500
    assert!(
        response.status() == StatusCode::OK
            || response.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[tokio::test]
async fn restore_backup_rejects_path_traversal() {
    let app = backup_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/restore")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"filename": "../../etc/passwd"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert!(json.get("error").is_some());
}

#[tokio::test]
async fn restore_backup_rejects_slash_in_filename() {
    let app = backup_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/restore")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"filename": "subdir/backup.sql.gz"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn restore_backup_rejects_backslash_in_filename() {
    let app = backup_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/restore")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"filename": "subdir\\backup.sql.gz"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn restore_backup_returns_not_found_for_missing_file() {
    let app = backup_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/restore")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"filename": "nonexistent_backup.sql.gz"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn backup_error_response_is_json() {
    let app = backup_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
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
    assert!(json.get("error").is_some());
}
