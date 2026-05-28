use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

use crate::error::AppError;
use crate::state::AppState;
use crate::tasks::backup::{create_pg_dump, list_backup_files, restore_from_dump};

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct BackupInfo {
    pub filename: String,
    pub size_bytes: u64,
    pub created_at: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct BackupResponse {
    pub message: String,
    pub filename: String,
    pub size_bytes: u64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct BackupListResponse {
    pub backups: Vec<BackupInfo>,
    pub total: usize,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RestoreRequest {
    pub filename: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RestoreResponse {
    pub message: String,
    pub filename: String,
}

/// POST /api/v1/backup
///
/// Create a new database backup. Requires admin authentication.
#[utoipa::path(
    post,
    path = "/api/v1/backup",
    responses(
        (status = 200, description = "Backup created successfully", body = BackupResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "backup"
)]
pub async fn create_backup(
    State(state): State<Arc<AppState>>,
) -> Result<Json<BackupResponse>, AppError> {
    let backup_dir = PathBuf::from(&state.config.backup_dir);
    let database_url = &state.config.database_url;

    let (filename, size) = create_pg_dump(database_url, &backup_dir)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Backup failed: {}", e)))?;

    Ok(Json(BackupResponse {
        message: "Backup created successfully".to_string(),
        filename,
        size_bytes: size,
    }))
}

/// GET /api/v1/backup/list
///
/// List all available backups. Requires admin authentication.
#[utoipa::path(
    get,
    path = "/api/v1/backup/list",
    responses(
        (status = 200, description = "Backup list retrieved", body = BackupListResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "backup"
)]
pub async fn list_backups(
    State(state): State<Arc<AppState>>,
) -> Result<Json<BackupListResponse>, AppError> {
    let backup_dir = PathBuf::from(&state.config.backup_dir);

    let backups = list_backup_files(&backup_dir)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to list backups: {}", e)))?;

    let total = backups.len();

    Ok(Json(BackupListResponse { backups, total }))
}

/// POST /api/v1/backup/restore
///
/// Restore database from a backup file. Requires admin authentication.
#[utoipa::path(
    post,
    path = "/api/v1/backup/restore",
    request_body = RestoreRequest,
    responses(
        (status = 200, description = "Database restored successfully", body = RestoreResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error")
    ),
    tag = "backup"
)]
pub async fn restore_backup(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RestoreRequest>,
) -> Result<Json<RestoreResponse>, AppError> {
    if req.filename.contains("..") || req.filename.contains('/') || req.filename.contains('\\') {
        return Err(AppError::BadRequest("Invalid filename".to_string()));
    }

    let backup_dir = PathBuf::from(&state.config.backup_dir);
    let backup_path = backup_dir.join(&req.filename);

    if !backup_path.exists() {
        return Err(AppError::NotFound(format!("Backup file not found: {}", req.filename)));
    }

    let database_url = &state.config.database_url;

    restore_from_dump(database_url, &backup_path)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Restore failed: {}", e)))?;

    Ok(Json(RestoreResponse {
        message: "Database restored successfully".to_string(),
        filename: req.filename,
    }))
}

/// DELETE /api/v1/backup/:filename
///
/// Delete a backup file. Requires admin authentication.
#[utoipa::path(
    delete,
    path = "/api/v1/backup/{filename}",
    params(
        ("filename" = String, Path, description = "Backup filename to delete")
    ),
    responses(
        (status = 204, description = "Backup file deleted successfully"),
        (status = 400, description = "Invalid filename"),
        (status = 404, description = "Backup file not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "backup"
)]
pub async fn delete_backup(
    State(state): State<Arc<AppState>>,
    Path(filename): Path<String>,
) -> Result<StatusCode, AppError> {
    let backup_dir = PathBuf::from(&state.config.backup_dir);
    let file_path = backup_dir.join(&filename);

    // Path traversal protection
    if !file_path.starts_with(&backup_dir) {
        return Err(AppError::BadRequest("Invalid filename".to_string()));
    }

    if !file_path.exists() {
        return Err(AppError::NotFound("Backup file not found".to_string()));
    }

    tokio::fs::remove_file(&file_path)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to delete backup: {}", e)))?;
    Ok(StatusCode::NO_CONTENT)
}
