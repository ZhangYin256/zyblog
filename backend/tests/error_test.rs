use axum::http::StatusCode;
use axum::response::IntoResponse;
use zyblog::error::AppError;

#[test]
fn app_error_not_found_status() {
    let err = AppError::NotFound("Post not found".to_string());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn app_error_bad_request_status() {
    let err = AppError::BadRequest("Invalid input".to_string());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[test]
fn app_error_internal_status() {
    let err = AppError::Internal(anyhow::anyhow!("something broke"));
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn app_error_payload_too_large_status() {
    let err = AppError::PayloadTooLarge("file too big".to_string());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[test]
fn app_error_conflict_status() {
    let err = AppError::Conflict("duplicate entry".to_string());
    let response = err.into_response();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[test]
fn app_error_display() {
    let err = AppError::NotFound("item".to_string());
    assert_eq!(format!("{}", err), "Not found: item");

    let err = AppError::BadRequest("bad".to_string());
    assert_eq!(format!("{}", err), "Bad request: bad");

    let err = AppError::PayloadTooLarge("big".to_string());
    assert_eq!(format!("{}", err), "Payload too large: big");

    let err = AppError::Conflict("dup".to_string());
    assert_eq!(format!("{}", err), "Conflict: dup");
}

#[test]
fn app_error_from_anyhow() {
    let anyhow_err = anyhow::anyhow!("test error");
    let app_err: AppError = anyhow_err.into();
    assert!(matches!(app_err, AppError::Internal(_)));
}

#[test]
fn app_error_from_db_err() {
    let db_err = sea_orm::DbErr::Custom("db error".to_string());
    let app_err: AppError = db_err.into();
    assert!(matches!(app_err, AppError::Database(_)));
}
