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

/// Request body for creating a post.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image: Option<String>,
    pub status: Option<String>, // "draft" or "published"
}

/// Request body for updating a post.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<String>,
    pub cover_image: Option<String>,
    pub status: Option<String>,
}

/// Query parameters for listing posts.
#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub status: Option<String>,
}

/// Response representation of a post.
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

/// Paginated list response.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct PostListResponse {
    pub items: Vec<PostResponse>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

/// Response for a created post (includes todos if any).
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct CreatePostResponse {
    #[serde(flatten)]
    pub post: PostResponse,
    pub todos_created: Vec<TodoItemResponse>,
}

/// Response for a todo item.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TodoItemResponse {
    pub id: i32,
    pub post_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: String,
}

/// Extract #todo tags from content.
/// Returns a list of todo titles extracted from patterns like #todo Some task
fn extract_todos(content: &str) -> Vec<String> {
    let mut todos = Vec::new();
    for line in content.lines() {
        let mut remaining = line;
        while let Some(pos) = remaining.find("#todo") {
            let after_tag = &remaining[pos + 5..];
            // Extract text until next #todo, end of line, or period
            let title = if after_tag.starts_with(' ') {
                let text = after_tag.trim_start();
                // Take until next #todo or end of line
                let end = text
                    .find("#todo")
                    .unwrap_or(text.len());
                text[..end].trim().to_string()
            } else if after_tag.is_empty() || after_tag.starts_with(|c: char| !c.is_alphanumeric())
            {
                // #todo at end or followed by non-alphanumeric
                String::from("Untitled todo")
            } else {
                // #todofix - not a valid #todo tag, skip
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

/// Generate a slug from a title.
fn generate_slug(title: &str) -> String {
    slug::slugify(title)
}

/// Convert a post model to a response.
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

/// POST /api/v1/posts - Create a new post.
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

    // Validate title is not empty
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

    // Parse #todo tags and create todo items with post_id
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

/// GET /api/v1/posts - List posts with pagination.
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

    // Filter by status if provided
    if let Some(ref status) = query.status {
        let published = status == "published";
        select = select.filter(post::Column::Published.eq(published));
    }

    // Count total
    let total = select.clone().count(db).await?;

    // Paginate and order by created_at desc
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

/// GET /api/v1/posts/:id - Get a single post.
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

/// PUT /api/v1/posts/:id - Update a post.
///
/// If the post is published and has #todo items, subscribers are notified.
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
        // Read from active model - we need to get the value
        // Since we're setting it conditionally, let's use the was_published
        was_published
    };

    active_model.updated_at = Set(Utc::now());

    let updated_post = active_model.update(db).await?;

    // Re-parse #todo tags: delete old ones, create new ones
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

    // Notify subscribers if post is now published and wasn't before
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

/// DELETE /api/v1/posts/:id - Delete a post.
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

    // Delete associated todo items first
    todo_item::Entity::delete_many()
        .filter(todo_item::Column::PostId.eq(id))
        .exec(db)
        .await?;

    post.delete(db).await?;

    Ok(axum::http::StatusCode::NO_CONTENT)
}

/// GET /api/v1/posts/:id/todos - Get todo items for a post.
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

    // Verify post exists
    let _post = post::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Post with id {} not found", id)))?;

    // Filter todos by post_id
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
