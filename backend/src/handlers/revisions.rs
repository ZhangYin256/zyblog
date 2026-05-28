use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::{post, post_revision};
use crate::state::AppState;

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

/// Diff 查询参数
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct DiffQuery {
    pub from: i32,
    pub to: i32,
}

/// 修订版本响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RevisionResponse {
    pub id: i32,
    pub post_id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub cover_image: Option<String>,
    pub version: i32,
    pub created_by: Option<i32>,
    pub created_at: String,
}

/// 修订版本列表响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct RevisionListResponse {
    pub items: Vec<RevisionResponse>,
    pub total: u64,
}

/// Diff 响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DiffResponse {
    pub diff: String,
    pub from_version: i32,
    pub to_version: i32,
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn revision_to_response(m: &post_revision::Model) -> RevisionResponse {
    RevisionResponse {
        id: m.id,
        post_id: m.post_id,
        title: m.title.clone(),
        content: m.content.clone(),
        excerpt: m.excerpt.clone(),
        cover_image: m.cover_image.clone(),
        version: m.version,
        created_by: m.created_by,
        created_at: m.created_at.to_rfc3339(),
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

/// POST /api/v1/posts/:id/revisions — 创建修订版本快照
#[utoipa::path(
    post,
    path = "/api/v1/posts/{id}/revisions",
    params(("id" = i32, Path, description = "Post ID")),
    responses(
        (status = 201, description = "Revision created", body = RevisionResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "revisions"
)]
pub async fn create_revision(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<(axum::http::StatusCode, Json<RevisionResponse>), AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 查找文章
    let existing_post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    // 计算下一个版本号
    let max_version = post_revision::Entity::find()
        .filter(post_revision::Column::PostId.eq(post_id))
        .order_by_desc(post_revision::Column::Version)
        .one(db)
        .await?
        .map(|r| r.version)
        .unwrap_or(0);

    let new_version = max_version + 1;
    let now = Utc::now();

    // 创建修订版本
    let new_revision = post_revision::ActiveModel {
        post_id: Set(post_id),
        title: Set(Some(existing_post.title.clone())),
        content: Set(Some(existing_post.content.clone())),
        excerpt: Set(existing_post.excerpt.clone()),
        cover_image: Set(existing_post.cover_image.clone()),
        version: Set(new_version),
        created_by: Set(existing_post.author_id),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_revision.insert(db).await?;

    // 更新文章的 current_version
    let mut post_active: post::ActiveModel = existing_post.into();
    post_active.current_version = Set(new_version);
    post_active.updated_at = Set(now);
    post_active.update(db).await?;

    tracing::info!("Revision {} created for post {}", new_version, post_id);

    Ok((axum::http::StatusCode::CREATED, Json(revision_to_response(&model))))
}

/// GET /api/v1/posts/:id/revisions — 获取修订版本列表
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}/revisions",
    params(("id" = i32, Path, description = "Post ID")),
    responses(
        (status = 200, description = "List of revisions", body = RevisionListResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "revisions"
)]
pub async fn list_revisions(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
) -> Result<Json<RevisionListResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证文章存在
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    let total = post_revision::Entity::find()
        .filter(post_revision::Column::PostId.eq(post_id))
        .count(db)
        .await?;

    let revisions = post_revision::Entity::find()
        .filter(post_revision::Column::PostId.eq(post_id))
        .order_by_desc(post_revision::Column::Version)
        .all(db)
        .await?;

    let items = revisions.iter().map(revision_to_response).collect();

    Ok(Json(RevisionListResponse { items, total }))
}

/// GET /api/v1/posts/:id/revisions/:rev_id — 获取特定修订版本
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}/revisions/{rev_id}",
    params(
        ("id" = i32, Path, description = "Post ID"),
        ("rev_id" = i32, Path, description = "Revision ID")
    ),
    responses(
        (status = 200, description = "Revision found", body = RevisionResponse),
        (status = 404, description = "Revision not found")
    ),
    tag = "revisions"
)]
pub async fn get_revision(
    State(state): State<Arc<AppState>>,
    Path((post_id, rev_id)): Path<(i32, i32)>,
) -> Result<Json<RevisionResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    let revision = post_revision::Entity::find_by_id(rev_id)
        .filter(post_revision::Column::PostId.eq(post_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Revision {} not found for post {}", rev_id, post_id)))?;

    Ok(Json(revision_to_response(&revision)))
}

/// POST /api/v1/posts/:id/revisions/:rev_id/rollback — 回滚到指定修订版本
#[utoipa::path(
    post,
    path = "/api/v1/posts/{id}/revisions/{rev_id}/rollback",
    params(
        ("id" = i32, Path, description = "Post ID"),
        ("rev_id" = i32, Path, description = "Revision ID")
    ),
    responses(
        (status = 200, description = "Rollback successful, new revision created", body = RevisionResponse),
        (status = 404, description = "Post or revision not found")
    ),
    tag = "revisions"
)]
pub async fn rollback_revision(
    State(state): State<Arc<AppState>>,
    Path((post_id, rev_id)): Path<(i32, i32)>,
) -> Result<Json<RevisionResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 查找目标修订版本
    let target_revision = post_revision::Entity::find_by_id(rev_id)
        .filter(post_revision::Column::PostId.eq(post_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Revision {} not found for post {}", rev_id, post_id)))?;

    // 查找文章
    let existing_post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    // 计算下一个版本号
    let max_version = post_revision::Entity::find()
        .filter(post_revision::Column::PostId.eq(post_id))
        .order_by_desc(post_revision::Column::Version)
        .one(db)
        .await?
        .map(|r| r.version)
        .unwrap_or(0);

    let new_version = max_version + 1;
    let now = Utc::now();

    // 更新文章内容为目标修订版本的内容
    let mut post_active: post::ActiveModel = existing_post.into();
    if let Some(ref title) = target_revision.title {
        post_active.title = Set(title.clone());
    }
    if let Some(ref content) = target_revision.content {
        post_active.content = Set(content.clone());
    }
    post_active.excerpt = Set(target_revision.excerpt.clone());
    post_active.cover_image = Set(target_revision.cover_image.clone());
    post_active.current_version = Set(new_version);
    post_active.updated_at = Set(now);
    let updated_post = post_active.update(db).await?;

    // 创建新修订版本（保留历史记录）
    let new_revision = post_revision::ActiveModel {
        post_id: Set(post_id),
        title: Set(Some(updated_post.title.clone())),
        content: Set(Some(updated_post.content.clone())),
        excerpt: Set(updated_post.excerpt.clone()),
        cover_image: Set(updated_post.cover_image.clone()),
        version: Set(new_version),
        created_by: Set(updated_post.author_id),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_revision.insert(db).await?;

    tracing::info!(
        "Post {} rolled back to revision {}, new version {}",
        post_id,
        rev_id,
        new_version
    );

    Ok(Json(revision_to_response(&model)))
}

/// GET /api/v1/posts/:id/diff?from=REV1&to=REV2 — 生成两个修订版本之间的差异
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}/diff",
    params(
        ("id" = i32, Path, description = "Post ID"),
        ("from" = i32, Query, description = "Source revision ID"),
        ("to" = i32, Query, description = "Target revision ID")
    ),
    responses(
        (status = 200, description = "Diff generated", body = DiffResponse),
        (status = 404, description = "Post or revision not found")
    ),
    tag = "revisions"
)]
pub async fn diff_revisions(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<i32>,
    Query(query): Query<DiffQuery>,
) -> Result<Json<DiffResponse>, AppError> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("Database not available")))?;

    // 验证文章存在
    let _post = post::Entity::find_by_id(post_id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", post_id)))?;

    // 获取两个修订版本
    let from_revision = post_revision::Entity::find_by_id(query.from)
        .filter(post_revision::Column::PostId.eq(post_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Revision {} not found for post {}", query.from, post_id)))?;

    let to_revision = post_revision::Entity::find_by_id(query.to)
        .filter(post_revision::Column::PostId.eq(post_id))
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Revision {} not found for post {}", query.to, post_id)))?;

    // 获取内容
    let from_content = from_revision.content.as_deref().unwrap_or("");
    let to_content = to_revision.content.as_deref().unwrap_or("");

    // 生成 unified diff
    let diff = similar::TextDiff::from_lines(from_content, to_content);

    let mut diff_output = String::new();
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            match change.tag() {
                similar::ChangeTag::Delete => {
                    diff_output.push_str(&format!("-{}", change));
                }
                similar::ChangeTag::Insert => {
                    diff_output.push_str(&format!("+{}", change));
                }
                similar::ChangeTag::Equal => {
                    diff_output.push_str(&format!(" {}", change));
                }
            }
        }
    }

    Ok(Json(DiffResponse {
        diff: diff_output,
        from_version: from_revision.version,
        to_version: to_revision.version,
    }))
}
