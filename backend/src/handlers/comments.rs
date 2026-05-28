use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::{comment, post};
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// 提交评论的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateCommentRequest {
    pub author_name: String,
    pub author_email: Option<String>,
    pub content: String,
    pub referenced_content: Option<String>,
}

/// 评论响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CommentResponse {
    pub id: i32,
    pub post_id: i32,
    pub author_name: String,
    pub author_email: Option<String>,
    pub content: String,
    pub approved: bool,
    pub referenced_content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn comment_to_response(m: &comment::Model) -> CommentResponse {
    CommentResponse {
        id: m.id,
        post_id: m.post_id,
        author_name: m.author_name.clone(),
        author_email: m.author_email.clone(),
        content: m.content.clone(),
        approved: m.approved,
        referenced_content: m.referenced_content.clone(),
        created_at: m.created_at.to_rfc3339(),
        updated_at: m.updated_at.to_rfc3339(),
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/v1/posts/:id/comments — 提交评论（公开，无需认证）
///
/// 创建的评论默认为未审核状态（approved = false），
/// 需要管理员审核后才会对外显示。
#[utoipa::path(
    post,
    path = "/api/v1/posts/{id}/comments",
    params(("id" = i32, Path, description = "Post ID")),
    request_body = CreateCommentRequest,
    responses(
        (status = 201, description = "Comment submitted for moderation", body = CommentResponse),
        (status = 404, description = "Post not found"),
        (status = 400, description = "Bad request")
    ),
    tag = "comments"
)]
pub async fn create_comment(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
    Json(body): Json<CreateCommentRequest>,
) -> Result<(axum::http::StatusCode, Json<CommentResponse>), AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证文章存在
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    if body.author_name.trim().is_empty() {
        return Err(AppError::BadRequest("Author name is required".to_string()));
    }
    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("Content is required".to_string()));
    }

    let now = Utc::now();
    let new_comment = comment::ActiveModel {
        post_id: Set(post_id),
        author_name: Set(body.author_name),
        author_email: Set(body.author_email),
        content: Set(body.content),
        approved: Set(false),
        referenced_content: Set(body.referenced_content),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let model = new_comment.insert(db).await?;
    tracing::info!("Comment #{} submitted on post {} (pending moderation)", model.id, post_id);

    Ok((axum::http::StatusCode::CREATED, Json(comment_to_response(&model))))
}

/// GET /api/v1/posts/:id/comments — 获取文章已审核评论（公开）
///
/// 仅返回 approved = true 的评论，按创建时间降序排列。
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}/comments",
    params(("id" = i32, Path, description = "Post ID")),
    responses(
        (status = 200, description = "Approved comments for the post", body = Vec<CommentResponse>),
        (status = 404, description = "Post not found")
    ),
    tag = "comments"
)]
pub async fn list_approved_comments(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证文章存在
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    let comments = comment::Entity::find()
        .filter(comment::Column::PostId.eq(post_id))
        .filter(comment::Column::Approved.eq(true))
        .order_by_desc(comment::Column::CreatedAt)
        .all(db)
        .await?;

    let response = comments.iter().map(comment_to_response).collect();
    Ok(Json(response))
}

/// GET /api/v1/comments/pending — 获取待审核评论列表（需认证）
///
/// 返回所有文章中 approved = false 的评论，按创建时间降序排列。
#[utoipa::path(
    get,
    path = "/api/v1/comments/pending",
    responses(
        (status = 200, description = "Pending comments across all posts", body = Vec<CommentResponse>),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "comments"
)]
pub async fn list_pending_comments(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<CommentResponse>>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let comments = comment::Entity::find()
        .filter(comment::Column::Approved.eq(false))
        .order_by_desc(comment::Column::CreatedAt)
        .all(db)
        .await?;

    let response = comments.iter().map(comment_to_response).collect();
    Ok(Json(response))
}

/// PUT /api/v1/comments/:id/approve — 审核通过评论（需认证）
///
/// 将评论的 approved 字段设置为 true。
#[utoipa::path(
    put,
    path = "/api/v1/comments/{id}/approve",
    params(("id" = i32, Path, description = "Comment ID")),
    responses(
        (status = 200, description = "Comment approved", body = CommentResponse),
        (status = 404, description = "Comment not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "comments"
)]
pub async fn approve_comment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<CommentResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let existing = comment::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Comment with id {} not found", id)))?;

    let mut active: comment::ActiveModel = existing.into();
    active.approved = Set(true);
    active.updated_at = Set(Utc::now());

    let updated = active.update(db).await?;
    tracing::info!("Comment #{} approved", updated.id);

    Ok(Json(comment_to_response(&updated)))
}

/// DELETE /api/v1/comments/:id — 删除评论（需认证）
#[utoipa::path(
    delete,
    path = "/api/v1/comments/{id}",
    params(("id" = i32, Path, description = "Comment ID")),
    responses(
        (status = 204, description = "Comment deleted"),
        (status = 404, description = "Comment not found")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "comments"
)]
pub async fn delete_comment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<axum::http::StatusCode, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let comment = comment::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Comment with id {} not found", id)))?;

    comment.delete(db).await?;
    tracing::info!("Comment #{} deleted", id);

    Ok(axum::http::StatusCode::NO_CONTENT)
}
