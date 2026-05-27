use axum::extract::Multipart;
use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::error::AppError;

/// 最大文件大小：5MB
const MAX_FILE_SIZE: usize = 5 * 1024 * 1024;

/// 支持的图片 MIME 类型
const SUPPORTED_TYPES: &[&str] = &["image/jpeg", "image/png", "image/gif", "image/webp"];

/// 将 MIME 类型映射到文件扩展名
fn mime_to_extension(mime: &str) -> Option<&str> {
    match mime {
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        _ => None,
    }
}

/// POST /api/v1/images - 上传图片
pub async fn upload_image(mut multipart: Multipart) -> Result<Json<Value>, AppError> {
    // 确保上传目录存在
    let upload_dir = "static/uploads";
    tokio::fs::create_dir_all(upload_dir)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create upload dir: {e}")))?;

    // 从 multipart 提取文件字段
    let mut content_type: Option<String> = None;
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

            // 分块读取以避免 multer 内存限制
            while let Some(chunk) = field
                .chunk()
                .await
                .map_err(|e| AppError::PayloadTooLarge(format!("file too large: {e}")))?
            {
                file_data.extend_from_slice(&chunk);
                if file_data.len() > MAX_FILE_SIZE {
                    return Err(AppError::PayloadTooLarge(format!(
                        "file too large: {} bytes (max {} bytes)",
                        file_data.len(),
                        MAX_FILE_SIZE
                    )));
                }
            }
        }
    }

    // 必须有文件字段
    if !found_file {
        return Err(AppError::BadRequest("Missing 'file' field".to_string()));
    }
    let mime = content_type
        .ok_or_else(|| AppError::BadRequest("Missing content type".to_string()))?;

    // 验证文件格式
    if !SUPPORTED_TYPES.contains(&mime.as_str()) {
        return Err(AppError::BadRequest(format!(
            "unsupported format: {mime}. Supported: jpeg, png, gif, webp"
        )));
    }

    // 验证文件大小
    if file_data.len() > MAX_FILE_SIZE {
        return Err(AppError::PayloadTooLarge(format!(
            "file too large: {} bytes (max {} bytes)",
            file_data.len(),
            MAX_FILE_SIZE
        )));
    }

    // 生成唯一文件名
    let ext = mime_to_extension(&mime).unwrap(); // safe: already validated
    let filename = format!("{}.{}", Uuid::new_v4(), ext);
    let filepath = format!("{upload_dir}/{filename}");

    // 保存文件
    tokio::fs::write(&filepath, &file_data)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to save file: {e}")))?;

    tracing::info!("Image uploaded: {filepath}");

    let url = format!("/static/uploads/{filename}");
    Ok(Json(json!({
        "url": url,
        "filename": filename,
    })))
}
