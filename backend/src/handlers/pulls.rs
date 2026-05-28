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
use crate::models::{post, post_revision, pr_comment, pull_request};
use crate::state::AppState;
use crate::utils::{apply_fragments, validate_fragments, Fragment};

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// 创建 PR 的请求体（基于片段）
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreatePullRequest {
    pub user_email: String,
    pub fragments: Vec<Fragment>,
    pub message: Option<String>,
}

/// 更新 PR 状态的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdatePullRequest {
    pub status: String, // "open", "closed", "merged"
}

/// 添加行内评论的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct AddCommentRequest {
    pub fragment_index: i32,
    pub line: i32,
    pub content: String,
    pub user_email: String,
}

/// PR 响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct PullResponse {
    pub id: i32,
    pub post_id: i32,
    pub user_email: String,
    pub content: String,
    pub status: String,
    pub fragments: Option<serde_json::Value>,
    pub message: Option<String>,
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
    pub fragment_index: Option<i32>,
    pub line: Option<i32>,
    pub content: String,
    pub user_email: Option<String>,
    pub created_at: String,
}

/// 评论列表响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CommentListResponse {
    pub items: Vec<CommentResponse>,
    pub total: u64,
}

/// PR 合并响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ApplyPullResponse {
    pub pull_request: PullResponse,
    pub updated_post_id: i32,
    pub revision_version: i32,
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
        fragments: m.fragments.clone(),
        message: m.message.clone(),
        created_at: m.created_at.to_rfc3339(),
    }
}

fn pr_comment_to_response(m: &pr_comment::Model, user_email: Option<&str>) -> CommentResponse {
    CommentResponse {
        id: m.id,
        pull_request_id: m.pull_request_id,
        fragment_index: m.fragment_index,
        line: m.line,
        content: m.content.clone(),
        user_email: user_email.map(|s| s.to_string()).or_else(|| m.user_id.map(|id| format!("user_{}", id))),
        created_at: m.created_at.to_rfc3339(),
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/v1/posts/:id/pulls — 创建 PR（基于片段）
#[utoipa::path(
    post,
    path = "/api/v1/posts/{id}/pulls",
    params(("id" = i32, Path, description = "Post ID")),
    request_body = CreatePullRequest,
    responses(
        (status = 201, description = "Pull request created", body = PullResponse),
        (status = 404, description = "Post not found"),
        (status = 400, description = "Bad request / invalid fragments")
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
    let existing_post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    // 验证片段
    if body.fragments.is_empty() {
        return Err(AppError::BadRequest("At least one fragment is required".to_string()));
    }

    validate_fragments(&existing_post.content, &body.fragments)
        .map_err(AppError::BadRequest)?;

    // 序列化片段为 JSON
    let fragments_json = serde_json::to_value(&body.fragments)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to serialize fragments: {}", e)))?;

    // 生成 content 摘要（显示所有片段描述）
    let content_summary = body
        .fragments
        .iter()
        .enumerate()
        .filter_map(|(i, f)| {
            f.description
                .as_deref()
                .map(|d| format!("[{}] {}", i, d))
        })
        .collect::<Vec<_>>()
        .join("; ");

    let now = Utc::now();
    let new_pull = pull_request::ActiveModel {
        post_id: Set(post_id),
        user_email: Set(body.user_email),
        content: Set(if content_summary.is_empty() {
            format!("{} fragment(s)", body.fragments.len())
        } else {
            content_summary
        }),
        status: Set("open".to_string()),
        fragments: Set(Some(fragments_json)),
        message: Set(body.message),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_pull.insert(db).await?;
    tracing::info!("Pull #{} created on post {} with {} fragments", model.id, post_id, body.fragments.len());

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

/// POST /api/v1/pulls/:id/apply — 合并 PR（应用片段到文章）
#[utoipa::path(
    post,
    path = "/api/v1/pulls/{id}/apply",
    params(("id" = i32, Path, description = "Pull ID")),
    responses(
        (status = 200, description = "PR applied, post updated", body = ApplyPullResponse),
        (status = 404, description = "Pull or post not found"),
        (status = 400, description = "PR not open or fragments invalid")
    ),
    tag = "pulls"
)]
pub async fn apply_pull(
    State(state): State<Arc<AppState>>,
    Path(pull_id): Path<i32>,
) -> Result<Json<ApplyPullResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 获取 PR
    let existing_pull = pull_request::Entity::find_by_id(pull_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Pull with id {} not found", pull_id)))?;

    if existing_pull.status != "open" {
        return Err(AppError::BadRequest(format!(
            "Pull #{} is not open (status: {})",
            pull_id, existing_pull.status
        )));
    }

    // 解析片段
    let fragments: Vec<Fragment> = match &existing_pull.fragments {
        Some(json_val) => serde_json::from_value(json_val.clone())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to parse fragments: {}", e)))?,
        None => return Err(AppError::BadRequest("Pull request has no fragments".to_string())),
    };

    // 获取文章
    let existing_post = post::Entity::find_by_id(existing_pull.post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", existing_pull.post_id)))?;

    // 应用片段
    let new_content = apply_fragments(&existing_post.content, &fragments)
        .map_err(AppError::BadRequest)?;

    let now = Utc::now();

    // 创建修订版本快照（合并前的文章状态）
    let max_version = post_revision::Entity::find()
        .filter(post_revision::Column::PostId.eq(existing_pull.post_id))
        .order_by_desc(post_revision::Column::Version)
        .one(db)
        .await?
        .map(|r| r.version)
        .unwrap_or(0);

    let new_version = max_version + 1;

    let new_revision = post_revision::ActiveModel {
        post_id: Set(existing_pull.post_id),
        title: Set(Some(existing_post.title.clone())),
        content: Set(Some(existing_post.content.clone())),
        excerpt: Set(existing_post.excerpt.clone()),
        cover_image: Set(existing_post.cover_image.clone()),
        version: Set(new_version),
        created_by: Set(existing_post.author_id),
        created_at: Set(now),
        ..Default::default()
    };
    let _revision_model = new_revision.insert(db).await?;

    // 更新文章内容
    let mut post_active: post::ActiveModel = existing_post.into();
    post_active.content = Set(new_content);
    post_active.current_version = Set(new_version);
    post_active.updated_at = Set(now);
    post_active.update(db).await?;

    // 更新 PR 状态为 merged
    let mut pull_active: pull_request::ActiveModel = existing_pull.into();
    pull_active.status = Set("merged".to_string());
    let updated_pull = pull_active.update(db).await?;

    tracing::info!(
        "Pull #{} applied to post {}, revision {}",
        pull_id,
        updated_pull.post_id,
        new_version
    );

    Ok(Json(ApplyPullResponse {
        pull_request: pull_to_response(&updated_pull),
        updated_post_id: updated_pull.post_id,
        revision_version: new_version,
    }))
}

/// POST /api/v1/pulls/:id/comments — 添加行内评论
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
    let new_comment = pr_comment::ActiveModel {
        pull_request_id: Set(pull_id),
        fragment_index: Set(Some(body.fragment_index)),
        line: Set(Some(body.line)),
        content: Set(body.content),
        user_id: Set(None),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_comment.insert(db).await?;
    tracing::info!(
        "Inline comment #{} added to pull #{} (fragment {}, line {})",
        model.id,
        pull_id,
        body.fragment_index,
        body.line
    );

    Ok((
        axum::http::StatusCode::CREATED,
        Json(pr_comment_to_response(&model, Some(&body.user_email))),
    ))
}

/// GET /api/v1/pulls/:id/comments — 获取 PR 的所有行内评论
#[utoipa::path(
    get,
    path = "/api/v1/pulls/{id}/comments",
    params(("id" = i32, Path, description = "Pull ID")),
    responses(
        (status = 200, description = "List of comments", body = CommentListResponse),
        (status = 404, description = "Pull not found")
    ),
    tag = "pulls"
)]
pub async fn list_comments(
    State(state): State<Arc<AppState>>,
    Path(pull_id): Path<i32>,
) -> Result<Json<CommentListResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证 PR 存在
    let _pull = pull_request::Entity::find_by_id(pull_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Pull with id {} not found", pull_id)))?;

    let total = pr_comment::Entity::find()
        .filter(pr_comment::Column::PullRequestId.eq(pull_id))
        .count(db)
        .await?;

    let comments = pr_comment::Entity::find()
        .filter(pr_comment::Column::PullRequestId.eq(pull_id))
        .order_by_asc(pr_comment::Column::CreatedAt)
        .all(db)
        .await?;

    let items = comments
        .iter()
        .map(|c| pr_comment_to_response(c, None))
        .collect();

    Ok(Json(CommentListResponse { items, total }))
}
