use axum::{
    extract::{Query, State},
    http::header,
    response::{IntoResponse, Response},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::post;
use crate::state::AppState;

/// Query parameters for the export endpoint.
#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    /// Export format: "json" (default) or "csv".
    pub format: Option<String>,
    /// Filter by status: "published" or "draft". Omit for all.
    pub status: Option<String>,
}

/// A single exported post record.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ExportedPost {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// GET /api/v1/export/posts
///
/// Export all posts as JSON or CSV. Requires admin authentication.
#[utoipa::path(
    get,
    path = "/api/v1/export/posts",
    params(
        ("format" = Option<String>, Query, description = "Export format: json or csv (default: json)"),
        ("status" = Option<String>, Query, description = "Filter by status: published or draft"),
    ),
    responses(
        (status = 200, description = "Posts exported successfully", body = Vec<ExportedPost>),
        (status = 500, description = "Internal server error")
    ),
    tag = "export"
)]
pub async fn export_posts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ExportQuery>,
) -> Result<Response, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let mut select = post::Entity::find();

    if let Some(ref status) = query.status {
        let published = status == "published";
        select = select.filter(post::Column::Published.eq(published));
    }

    let posts = select
        .order_by_desc(post::Column::CreatedAt)
        .all(db)
        .await?;

    let exported: Vec<ExportedPost> = posts
        .iter()
        .map(|p| ExportedPost {
            id: p.id,
            title: p.title.clone(),
            slug: p.slug.clone(),
            content: p.content.clone(),
            excerpt: p.excerpt.clone(),
            cover_image: p.cover_image.clone(),
            status: if p.published { "published" } else { "draft" }.to_string(),
            created_at: p.created_at.to_rfc3339(),
            updated_at: p.updated_at.to_rfc3339(),
        })
        .collect();

    let format = query.format.as_deref().unwrap_or("json");

    match format {
        "csv" => export_as_csv(exported),
        _ => export_as_json(exported),
    }
}

fn export_as_json(posts: Vec<ExportedPost>) -> Result<Response, AppError> {
    let body = serde_json::to_string_pretty(&posts)
        .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    Ok((
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        body,
    )
        .into_response())
}

fn export_as_csv(posts: Vec<ExportedPost>) -> Result<Response, AppError> {
    let mut wtr = csv::Writer::from_writer(Vec::new());
    for post in &posts {
        wtr.serialize(post).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    }
    let data = wtr.into_inner().map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
    let body = String::from_utf8(data).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;

    Ok((
        [(header::CONTENT_TYPE, "text/csv; charset=utf-8")],
        body,
    )
        .into_response())
}
