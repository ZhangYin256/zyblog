use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::{post, todo_item};
use crate::state::AppState;
use crate::tasks::email::notify_subscribers_on_update;

/// 创建文章的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image: Option<String>,
    pub status: Option<String>, // "draft" or "published"
}

/// 更新文章的请求体
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub cover_image: Option<String>,
    pub status: Option<String>,
}

/// 文章列表查询参数
#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub status: Option<String>,
}

/// 文章响应表示
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct PostResponse {
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

/// 分页列表响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct PostListResponse {
    pub items: Vec<PostResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

/// 创建文章的响应（包含待办事项）
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CreatePostResponse {
    #[serde(flatten)]
    pub post: PostResponse,
    pub todos_created: Vec<TodoItemResponse>,
}

/// 待办事项响应
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TodoItemResponse {
    pub id: i32,
    pub post_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
}

/// 从内容中提取 #todo 标签
/// 返回从 #todo 模式中提取的待办事项标题列表
fn extract_todos(content: &str) -> Vec<String> {
    let mut todos = Vec::new();
    for line in content.lines() {
        let mut remaining = line;
        while let Some(pos) = remaining.find("#todo") {
            let after_tag = &remaining[pos + 5..];
            // 提取文本直到下一个 #todo、行尾或句号
            let title = if after_tag.starts_with(' ') {
                let text = after_tag.trim_start();
                // 取到下一个 #todo 或行尾
                let end = text
                    .find("#todo")
                    .unwrap_or(text.len());
                text[..end].trim().to_string()
            } else if after_tag.is_empty() || after_tag.starts_with(|c: char| !c.is_alphanumeric())
            {
                // 行尾的 #todo 或后跟非字母数字字符
                String::from("Untitled todo")
            } else {
                // #todofix - 不是有效的 #todo 标签，跳过
                remaining = &remaining[pos + 5..];
                continue;
            };

            if !title.is_empty() {
                todos.push(title.clone());
            }
            remaining = &remaining[pos + 5..];
        }
    }
    todos
}

/// 从标题生成 slug
fn generate_slug(title: &str) -> String {
    slug::slugify(title)
}

/// 将文章模型转换为响应
fn post_to_response(post: &post::Model) -> PostResponse {
    PostResponse {
        id: post.id,
        title: post.title.clone(),
        slug: post.slug.clone(),
        content: post.content.clone(),
        excerpt: post.excerpt.clone(),
        cover_image: post.cover_image.clone(),
        status: if post.published {
            "published".to_string()
        } else {
            "draft".to_string()
        },
        created_at: post.created_at.to_rfc3339(),
        updated_at: post.updated_at.to_rfc3339(),
    }
}

fn todo_to_response(todo: &todo_item::Model) -> TodoItemResponse {
    TodoItemResponse {
        id: todo.id,
        post_id: todo.post_id,
        title: todo.title.clone(),
        description: todo.description.clone(),
        completed: todo.completed,
        created_at: todo.created_at.to_rfc3339(),
    }
}

/// POST /api/v1/posts - 创建新文章
#[utoipa::path(
    post,
    path = "/api/v1/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created successfully", body = CreatePostResponse),
        (status = 400, description = "Bad request")
    ),
    tag = "posts"
)]
pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreatePostRequest>,
) -> Result<(axum::http::StatusCode, Json<CreatePostResponse>), AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // 验证标题不为空
    if body.title.trim().is_empty() {
        return Err(AppError::BadRequest("Title is required".to_string()));
    }

    let slug = generate_slug(&body.title);
    let published = body
        .status
        .as_deref()
        .map(|s| s == "published")
        .unwrap_or(false);

    let now = Utc::now();
    let new_post = post::ActiveModel {
        title: Set(body.title.clone()),
        slug: Set(slug),
        content: Set(body.content.clone()),
        excerpt: Set(body.excerpt),
        cover_image: Set(body.cover_image),
        published: Set(published),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let post = new_post.insert(db).await?;

    // 解析 #todo 标签并创建待办事项
    let todo_titles = extract_todos(&body.content);
    let mut created_todos = Vec::new();

    for title in todo_titles {
        let new_todo = todo_item::ActiveModel {
            post_id: Set(post.id),
            title: Set(title),
            description: Set(None),
            completed: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };

        let todo = new_todo.insert(db).await?;
        created_todos.push(todo_to_response(&todo));
    }

    let response = CreatePostResponse {
        post: post_to_response(&post),
        todos_created: created_todos,
    };

    Ok((axum::http::StatusCode::CREATED, Json(response)))
}

/// GET /api/v1/posts - 分页获取文章列表
#[utoipa::path(
    get,
    path = "/api/v1/posts",
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default: 10, max: 100)"),
        ("status" = Option<String>, Query, description = "Filter by status: published or draft"),
    ),
    responses(
        (status = 200, description = "List of posts", body = PostListResponse)
    ),
    tag = "posts"
)]
pub async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<PostListResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1, 100);

    let mut select = post::Entity::find();

    // 按状态筛选（如果提供）
    if let Some(ref status) = query.status {
        let published = status == "published";
        select = select.filter(post::Column::Published.eq(published));
    }

    // 统计总数
    let total = select.clone().count(db).await?;

    // 分页并按创建时间降序排列
    let posts = select
        .order_by_desc(post::Column::CreatedAt)
        .paginate(db, per_page)
        .fetch_page(page - 1) // SeaORM uses 0-based pagination
        .await?;

    let items = posts.iter().map(post_to_response).collect();

    Ok(Json(PostListResponse {
        items,
        total,
        page,
        per_page,
    }))
}

/// GET /api/v1/posts/:id - 获取单篇文章
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}",
    params(
        ("id" = i32, Path, description = "Post ID"),
    ),
    responses(
        (status = 200, description = "Post found", body = PostResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<PostResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let post = post::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", id)))?;

    Ok(Json(post_to_response(&post)))
}

/// PUT /api/v1/posts/:id - 更新文章
///
/// 如果文章已发布且包含 #todo 项目，将通知订阅者
#[utoipa::path(
    put,
    path = "/api/v1/posts/{id}",
    params(
        ("id" = i32, Path, description = "Post ID"),
    ),
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updated", body = PostResponse),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UpdatePostRequest>,
) -> Result<Json<PostResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let existing_post = post::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", id)))?;

    let was_published = existing_post.published;

    let mut active_model: post::ActiveModel = existing_post.into();

    if let Some(title) = body.title {
        if title.trim().is_empty() {
            return Err(AppError::BadRequest("Title cannot be empty".to_string()));
        }
        active_model.slug = Set(generate_slug(&title));
        active_model.title = Set(title);
    }

    let new_content = if let Some(content) = body.content {
        active_model.content = Set(content.clone());
        Some(content)
    } else {
        None
    };

    if let Some(excerpt) = body.excerpt {
        active_model.excerpt = Set(Some(excerpt));
    }

    if let Some(cover_image) = body.cover_image {
        active_model.cover_image = Set(Some(cover_image));
    }

    let now_published = if let Some(status) = body.status {
        let p = status == "published";
        active_model.published = Set(p);
        p
    } else {
        // 从活动模型读取 - 需要获取值
        // 由于是条件性设置，使用 was_published
        was_published
    };

    active_model.updated_at = Set(Utc::now());

    let updated_post = active_model.update(db).await?;

    // 重新解析 #todo 标签：删除旧的，创建新的
    if let Some(ref content) = new_content {
        todo_item::Entity::delete_many()
            .filter(todo_item::Column::PostId.eq(id))
            .exec(db)
            .await?;

        let todo_titles = extract_todos(content);
        for title in todo_titles {
            let new_todo = todo_item::ActiveModel {
                post_id: Set(id),
                title: Set(title),
                description: Set(None),
                completed: Set(false),
                created_at: Set(Utc::now()),
                updated_at: Set(Utc::now()),
                ..Default::default()
            };
            new_todo.insert(db).await?;
        }
    }

    // 如果文章刚发布且之前未发布，通知订阅者
    if now_published && !was_published {
        let state_clone = state.clone();
        let post_title = updated_post.title.clone();
        let post_id = updated_post.id;
        tokio::spawn(async move {
            if let Err(e) = notify_subscribers_on_update(&state_clone, post_id, &post_title).await {
                tracing::error!("Failed to notify subscribers: {}", e);
            }
        });
    }

    Ok(Json(post_to_response(&updated_post)))
}

/// DELETE /api/v1/posts/:id - 删除文章
#[utoipa::path(
    delete,
    path = "/api/v1/posts/{id}",
    params(
        ("id" = i32, Path, description = "Post ID"),
    ),
    responses(
        (status = 204, description = "Post deleted"),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<axum::http::StatusCode, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let post = post::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", id)))?;

    // 先删除关联的待办事项
    todo_item::Entity::delete_many()
        .filter(todo_item::Column::PostId.eq(id))
        .exec(db)
        .await?;

    post.delete(db).await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// GET /api/v1/posts/:id/todos - 获取文章的待办事项
#[utoipa::path(
    get,
    path = "/api/v1/posts/{id}/todos",
    params(
        ("id" = i32, Path, description = "Post ID"),
    ),
    responses(
        (status = 200, description = "Todo items for the post", body = Vec<TodoItemResponse>),
        (status = 404, description = "Post not found")
    ),
    tag = "posts"
)]
pub async fn get_post_todos(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<Vec<TodoItemResponse>>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // 验证文章存在
    let _post = post::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", id)))?;

    // 按文章 ID 筛选待办事项
    let todos = todo_item::Entity::find()
        .filter(todo_item::Column::PostId.eq(id))
        .order_by_desc(todo_item::Column::CreatedAt)
        .all(db)
        .await?;

    let response = todos.iter().map(todo_to_response).collect();

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_todos() {
        let content = "Fix this #todo and that #todo";
        let todos = extract_todos(content);
        assert_eq!(todos.len(), 2);
    }

    #[test]
    fn test_extract_todos_no_todos() {
        let content = "No todos here";
        let todos = extract_todos(content);
        assert!(todos.is_empty());
    }

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World"), "hello-world");
        assert_eq!(generate_slug("Test Post!"), "test-post");
        assert_eq!(generate_slug("  Spaces  "), "spaces");
    }

    #[test]
    fn test_extract_todos_multiple_lines() {
        let content = "#todo Task 1\n#todo Task 2\n#todo Task 3";
        let todos = extract_todos(content);
        assert_eq!(todos.len(), 3);
        assert_eq!(todos[0], "Task 1");
        assert_eq!(todos[1], "Task 2");
        assert_eq!(todos[2], "Task 3");
    }
}
