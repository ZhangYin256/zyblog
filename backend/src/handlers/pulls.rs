use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::{post, pull_request, pull_request_comment};
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// 创建 PR 的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreatePullRequest {
    pub user_email: String,
    pub content: String,
}

/// 更新 PR 状态的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdatePullRequest {
    pub status: String, // "open", "closed", "merged"
}

/// 添加评论的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct AddCommentRequest {
    pub user_email: String,
    pub content: String,
}

/// PR 响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct PullResponse {
    pub id: i32,
    pub post_id: i32,
    pub user_email: String,
    pub content: String,
    pub status: String,
    pub created_at: String,
}

/// PR 列表响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct PullListResponse {
    pub items: Vec<PullResponse>,
    pub total: u64,
}

/// 评论响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CommentResponse {
    pub id: i32,
    pub pull_request_id: i32,
    pub user_email: String,
    pub content: String,
    pub created_at: String,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn pull_to_response(m: &pull_request::Model) -> PullResponse {
    PullResponse {
        id: m.id,
        post_id: m.post_id,
        user_email: m.user_email.clone(),
        content: m.content.clone(),
        status: m.status.clone(),
        created_at: m.created_at.to_rfc3339(),
    }
}

fn comment_to_response(m: &pull_request_comment::Model, user_email: &str) -> CommentResponse {
    CommentResponse {
        id: m.id,
        pull_request_id: m.pull_request_id,
        user_email: user_email.to_string(),
        content: m.content.clone(),
        created_at: m.created_at.to_rfc3339(),
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/v1/posts/:id/pulls — 创建 PR
#[utoipa::path(
    post,
    path = "/api/v1/posts/{id}/pulls",
    params(("id" = i32, Path, description = "Post ID")),
    request_body = CreatePullRequest,
    responses(
        (status = 201, description = "Pull request created", body = PullResponse),
        (status = 404, description = "Post not found"),
        (status = 400, description = "Bad request")
    ),
    tag = "pulls"
)]
pub async fn create_pull(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
    Json(body): Json<CreatePullRequest>,
) -> Result<(axum::http::StatusCode, Json<PullResponse>), AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证文章存在
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("Content is required".to_string()));
    }

    let now = Utc::now();
    let new_pull = pull_request::ActiveModel {
        post_id: Set(post_id),
        user_email: Set(body.user_email),
        content: Set(body.content),
        status: Set("open".to_string()),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_pull.insert(db).await?;
    tracing::info!("Pull #{} created on post {}", model.id, post_id);

    Ok((axum::http::StatusCode::CREATED, Json(pull_to_response(&model))))
}

/// GET /api/v1/posts/:id/pulls — 获取 PR 列表
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}/pulls",
    params(("id" = i32, Path, description = "Post ID")),
    responses(
        (status = 200, description = "List of pull requests", body = PullListResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "pulls"
)]
pub async fn list_pulls(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<Json<PullListResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证文章存在
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    let total = pull_request::Entity::find()
        .filter(pull_request::Column::PostId.eq(post_id))
        .count(db)
        .await?;

    let pulls = pull_request::Entity::find()
        .filter(pull_request::Column::PostId.eq(post_id))
        .order_by_desc(pull_request::Column::CreatedAt)
        .all(db)
        .await?;

    let items = pulls.iter().map(pull_to_response).collect();

    Ok(Json(PullListResponse { items, total }))
}

/// PUT /api/v1/pulls/:id — 更新 PR 状态
#[utoipa::path(
    put,
    path = "/api/v1/pulls/{id}",
    params(("id" = i32, Path, description = "Pull ID")),
    request_body = UpdatePullRequest,
    responses(
        (status = 200, description = "Pull request updated", body = PullResponse),
        (status = 404, description = "Pull not found"),
        (status = 400, description = "Invalid status value")
    ),
    tag = "pulls"
)]
pub async fn update_pull(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdatePullRequest>,
) -> Result<Json<PullResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let existing = pull_request::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Pull with id {} not found", id)))?;

    let valid = ["open", "closed", "merged"];
    if !valid.contains(&body.status.as_str()) {
        return Err(AppError::BadRequest(format!(
            "Invalid status '{}'. Must be one of: open, closed, merged",
            body.status
        )));
    }

    let mut active: pull_request::ActiveModel = existing.into();
    active.status = Set(body.status);

    let updated = active.update(db).await?;
    Ok(Json(pull_to_response(&updated)))
}

/// POST /api/v1/pulls/:id/comments — 添加评论
#[utoipa::path(
    post,
    path = "/api/v1/pulls/{id}/comments",
    params(("id" = i32, Path, description = "Pull ID")),
    request_body = AddCommentRequest,
    responses(
        (status = 201, description = "Comment added", body = CommentResponse),
        (status = 404, description = "Pull not found"),
        (status = 400, description = "Bad request")
    ),
    tag = "pulls"
)]
pub async fn add_comment(
    State(state): State<Arc<AppState>>,
    Path(pull_id): Path<i32>,
    Json(body): Json<AddCommentRequest>,
) -> Result<(axum::http::StatusCode, Json<CommentResponse>), AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证 PR 存在
    let _pull = pull_request::Entity::find_by_id(pull_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Pull with id {} not found", pull_id)))?;

    if body.content.trim().is_empty() {
        return Err(AppError::BadRequest("Content is required".to_string()));
    }

    let now = Utc::now();
    let new_comment = pull_request_comment::ActiveModel {
        pull_request_id: Set(pull_id),
        content: Set(body.content),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_comment.insert(db).await?;
    tracing::info!("Comment #{} added to pull #{}", model.id, pull_id);

    Ok((
        axum::http::StatusCode::CREATED,
        Json(comment_to_response(&model, &body.user_email)),
    ))
}
