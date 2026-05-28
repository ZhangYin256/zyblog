use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::media;
use crate::state::AppState;
use crate::storage::local::LocalStorage;
use crate::storage::StorageAdapter;

/// Maximum file size: 100MB
const MAX_FILE_SIZE: usize = 100 * 1024 * 1024;

/// Supported MIME types: images + videos
const SUPPORTED_TYPES: &[&str] = &[
    "image/jpeg",
    "image/png",
    "image/gif",
    "image/webp",
    "video/mp4",
    "video/webm",
];

/// Map MIME type to file extension
fn mime_to_extension(mime: &str) -> Option<&str> {
    match mime {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        "video/mp4" => Some("mp4"),
        "video/webm" => Some("webm"),
        _ => None,
    }
}

/// Create a LocalStorage instance for media files
fn create_storage() -> LocalStorage {
    LocalStorage::new(
        std::path::PathBuf::from("static/media"),
        "/static/media".to_string(),
    )
}

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// Media file response
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MediaResponse {
    pub id: i32,
    pub filename: String,
    pub original_name: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub url: String,
    pub created_at: String,
}

/// Paginated media list response
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct MediaListResponse {
    pub items: Vec<MediaResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

/// Media list query parameters
#[derive(Debug, Deserialize)]
pub struct ListMediaQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn media_to_response(m: &media::Model, url: &str) -> MediaResponse {
    MediaResponse {
        id: m.id,
        filename: m.filename.clone(),
        original_name: m.original_name.clone(),
        mime_type: m.mime_type.clone(),
        size_bytes: m.size_bytes,
        url: url.to_string(),
        created_at: m.created_at.to_rfc3339(),
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/v1/media — Upload a media file
#[utoipa::path(
    post,
    path = "/api/v1/media",
    request_body(content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Media uploaded successfully", body = MediaResponse),
        (status = 400, description = "Bad request"),
        (status = 413, description = "File too large")
    ),
    tag = "media"
)]
pub async fn upload_media(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<MediaResponse>), AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // Extract file from multipart
    let mut content_type: Option<String> = None;
    let mut original_name: Option<String> = None;
    let mut file_data: Vec<u8> = Vec::new();
    let mut found_file = false;

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart field: {e}")))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            found_file = true;
            content_type = field.content_type().map(|s| s.to_string());
            original_name = field.file_name().map(|s| s.to_string());

            // Read in chunks to avoid multer memory limit
            while let Some(chunk) = field
                .chunk()
                .await
                .map_err(|e| AppError::PayloadTooLarge(format!("File too large: {e}")))?
            {
                file_data.extend_from_slice(&chunk);
                if file_data.len() > MAX_FILE_SIZE {
                    return Err(AppError::PayloadTooLarge(format!(
                        "File too large: {} bytes (max {} bytes)",
                        file_data.len(),
                        MAX_FILE_SIZE
                    )));
                }
            }
        }
    }

    if !found_file {
        return Err(AppError::BadRequest("Missing 'file' field".to_string()));
    }

    let mime = content_type
        .ok_or_else(|| AppError::BadRequest("Missing content type".to_string()))?;

    // Validate file type
    if !SUPPORTED_TYPES.contains(&mime.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Unsupported format: {mime}. Supported: jpeg, png, gif, webp, mp4, webm"
        )));
    }

    // Validate file size
    if file_data.len() > MAX_FILE_SIZE {
        return Err(AppError::PayloadTooLarge(format!(
            "File too large: {} bytes (max {} bytes)",
            file_data.len(),
            MAX_FILE_SIZE
        )));
    }

    // Generate unique filename
    let ext = mime_to_extension(&mime).unwrap(); // safe: already validated
    let filename = format!("{}.{}", Uuid::new_v4(), ext);
    let storage_key = &filename;
    let original = original_name.unwrap_or_else(|| "unknown".to_string());

    // Upload via StorageAdapter
    let storage = create_storage();
    let url = storage
        .upload(storage_key, &file_data, &mime)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to save file: {e}")))?;

    // Save metadata to database
    let now = Utc::now();
    let new_media = media::ActiveModel {
        filename: Set(filename.clone()),
        original_name: Set(original),
        mime_type: Set(mime),
        size_bytes: Set(file_data.len() as i64),
        path: Set(filename.clone()),
        uploaded_by: Set(None),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_media.insert(db).await?;
    tracing::info!("Media uploaded: id={}, filename={}", model.id, model.filename);

    Ok((StatusCode::CREATED, Json(media_to_response(&model, &url))))
}

/// GET /api/v1/media — List media files (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/media",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default: 20, max: 100)"),
    ),
    responses(
        (status = 200, description = "List of media files", body = MediaListResponse)
    ),
    tag = "media"
)]
pub async fn list_media(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListMediaQuery>,
) -> Result<Json<MediaListResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let select = media::Entity::find();

    let total = select.clone().count(db).await?;

    let items = select
        .order_by_desc(media::Column::CreatedAt)
        .paginate(db, per_page)
        .fetch_page(page - 1) // SeaORM uses 0-based pagination
        .await?;

    let storage = create_storage();
    let response_items: Vec<MediaResponse> = items
        .iter()
        .map(|m| {
            let url = storage.get_url(&m.path);
            media_to_response(m, &url)
        })
        .collect();

    Ok(Json(MediaListResponse {
        items: response_items,
        total,
        page,
        per_page,
    }))
}

/// DELETE /api/v1/media/:id — Delete a media file
#[utoipa::path(
    delete,
    path = "/api/v1/media/{id}",
    params(("id" = i32, Path, description = "Media ID")),
    responses(
        (status = 204, description = "Media deleted"),
        (status = 404, description = "Media not found")
    ),
    tag = "media"
)]
pub async fn delete_media(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let media_item = media::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Media with id {} not found", id)))?;

    let filename = media_item.filename.clone();

    // Delete file from storage
    let storage = create_storage();
    if let Err(e) = storage.delete(&media_item.path).await {
        tracing::warn!("Failed to delete file from storage: {e}");
        // Continue to delete DB record even if file deletion fails
    }

    // Delete metadata from database
    media_item.delete(db).await?;
    tracing::info!("Media deleted: id={}, filename={}", id, filename);

    Ok(StatusCode::NO_CONTENT)
}
