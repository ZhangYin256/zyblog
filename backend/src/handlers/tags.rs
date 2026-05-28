use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::{post, post_tag, tag};
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// 创建标签的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateTagRequest {
    pub name: String,
}

/// 标签响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: String,
}

/// 批量分配标签的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct AssignTagsRequest {
    pub tag_ids: Vec<i32>,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn tag_to_response(m: &tag::Model) -> TagResponse {
    TagResponse {
        id: m.id,
        name: m.name.clone(),
        slug: m.slug.clone(),
        created_at: m.created_at.to_rfc3339(),
    }
}

// ---------------------------------------------------------------------------
// Tag CRUD handlers
// ---------------------------------------------------------------------------

/// POST /api/v1/tags — 创建标签
#[utoipa::path(
    post,
    path = "/api/v1/tags",
    request_body = CreateTagRequest,
    responses(
        (status = 201, description = "Tag created", body = TagResponse),
        (status = 409, description = "Tag name already exists"),
        (status = 400, description = "Bad request")
    ),
    tag = "tags"
)]
pub async fn create_tag(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateTagRequest>,
) -> Result<(StatusCode, Json<TagResponse>), AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    if body.name.trim().is_empty() {
        return Err(AppError::BadRequest("Tag name is required".to_string()));
    }

    let slug = slug::slugify(&body.name);

    // Check for duplicate name
    let existing = tag::Entity::find()
        .filter(tag::Column::Name.eq(&body.name))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Tag name already exists".to_string()));
    }

    let now = Utc::now();
    let new_tag = tag::ActiveModel {
        name: Set(body.name),
        slug: Set(slug),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_tag.insert(db).await?;
    tracing::info!("Tag '{}' created (id={})", model.name, model.id);

    Ok((StatusCode::CREATED, Json(tag_to_response(&model))))
}

/// GET /api/v1/tags — 获取所有标签
#[utoipa::path(
    get,
    path = "/api/v1/tags",
    responses(
        (status = 200, description = "List of tags", body = Vec<TagResponse>)
    ),
    tag = "tags"
)]
pub async fn list_tags(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<TagResponse>>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let tags = tag::Entity::find()
        .order_by_asc(tag::Column::Name)
        .all(db)
        .await?;

    let response: Vec<TagResponse> = tags.iter().map(tag_to_response).collect();
    Ok(Json(response))
}

/// DELETE /api/v1/tags/:id — 删除标签（同时删除关联的 post_tags）
#[utoipa::path(
    delete,
    path = "/api/v1/tags/{id}",
    params(("id" = i32, Path, description = "Tag ID")),
    responses(
        (status = 204, description = "Tag deleted"),
        (status = 404, description = "Tag not found")
    ),
    tag = "tags"
)]
pub async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let tag = tag::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Tag with id {} not found", id)))?;

    let tag_name = tag.name.clone();

    // Delete associated post_tags first
    post_tag::Entity::delete_many()
        .filter(post_tag::Column::TagId.eq(id))
        .exec(db)
        .await?;

    tag.delete(db).await?;
    tracing::info!("Tag '{}' (id={}) deleted", tag_name, id);

    Ok(StatusCode::NO_CONTENT)
}

// ---------------------------------------------------------------------------
// Post-Tag assignment handlers
// ---------------------------------------------------------------------------

/// POST /api/v1/posts/:id/tags — 为文章分配标签
#[utoipa::path(
    post,
    path = "/api/v1/posts/{id}/tags",
    params(("id" = i32, Path, description = "Post ID")),
    request_body = AssignTagsRequest,
    responses(
        (status = 200, description = "Tags assigned", body = Vec<TagResponse>),
        (status = 404, description = "Post not found")
    ),
    tag = "tags"
)]
pub async fn assign_tags_to_post(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
    Json(body): Json<AssignTagsRequest>,
) -> Result<Json<Vec<TagResponse>>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // Verify post exists
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    // For each tag_id, insert if not already associated
    for tag_id in &body.tag_ids {
        // Verify tag exists
        let _tag = tag::Entity::find_by_id(*tag_id)
            .one(db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Tag with id {} not found", tag_id)))?;

        // Check if association already exists
        let existing = post_tag::Entity::find_by_id((post_id, *tag_id))
            .one(db)
            .await?;

        if existing.is_none() {
            let new_post_tag = post_tag::ActiveModel {
                post_id: Set(post_id),
                tag_id: Set(*tag_id),
            };
            new_post_tag.insert(db).await?;
        }
    }

    // Return all tags for this post
    let post_tags = post_tag::Entity::find()
        .filter(post_tag::Column::PostId.eq(post_id))
        .all(db)
        .await?;

    let mut tags = Vec::new();
    for pt in post_tags {
        if let Some(t) = tag::Entity::find_by_id(pt.tag_id).one(db).await? {
            tags.push(tag_to_response(&t));
        }
    }

    tracing::info!("Tags assigned to post {}", post_id);
    Ok(Json(tags))
}

/// DELETE /api/v1/posts/:id/tags/:tag_id — 移除文章的标签
#[utoipa::path(
    delete,
    path = "/api/v1/posts/{id}/tags/{tag_id}",
    params(
        ("id" = i32, Path, description = "Post ID"),
        ("tag_id" = i32, Path, description = "Tag ID")
    ),
    responses(
        (status = 204, description = "Tag removed from post"),
        (status = 404, description = "Association not found")
    ),
    tag = "tags"
)]
pub async fn remove_tag_from_post(
    State(state): State<Arc<AppState>>,
    Path((post_id, tag_id)): Path<(i32, i32)>,
) -> Result<StatusCode, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let post_tag = post_tag::Entity::find_by_id((post_id, tag_id))
        .one(db)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "Tag {} is not assigned to post {}",
                tag_id, post_id
            ))
        })?;

    post_tag.delete(db).await?;
    tracing::info!("Tag {} removed from post {}", tag_id, post_id);

    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/posts/:id/tags — 获取文章的所有标签
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}/tags",
    params(("id" = i32, Path, description = "Post ID")),
    responses(
        (status = 200, description = "Tags for the post", body = Vec<TagResponse>),
        (status = 404, description = "Post not found")
    ),
    tag = "tags"
)]
pub async fn list_post_tags(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<Json<Vec<TagResponse>>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // Verify post exists
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    let post_tags = post_tag::Entity::find()
        .filter(post_tag::Column::PostId.eq(post_id))
        .all(db)
        .await?;

    let mut tags = Vec::new();
    for pt in post_tags {
        if let Some(t) = tag::Entity::find_by_id(pt.tag_id).one(db).await? {
            tags.push(tag_to_response(&t));
        }
    }

    Ok(Json(tags))
}
