use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::AppError;
use crate::models::{post, todo_item, todo_subscription};
use crate::state::AppState;
use crate::tasks::email::notify_todo_subscribers;

/// TODO item response with subscriber count
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TodoResponse {
    pub id: i32,
    pub post_id: i32,
    pub title: String,
    pub completed: bool,
    pub subscriber_count: i64,
}

/// TODO list response
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct TodoListResponse {
    pub items: Vec<TodoResponse>,
}

/// Subscribe request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SubscribeRequest {
    pub email: Option<String>,
    pub user_id: Option<i32>,
}

/// Unsubscribe request body
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UnsubscribeRequest {
    pub email: Option<String>,
    pub user_id: Option<i32>,
}

/// Check if the request has a valid admin token
fn is_admin_request(headers: &HeaderMap) -> bool {
    let admin_key = match std::env::var("ADMIN_KEY") {
        Ok(key) => key,
        Err(_) => return false,
    };

    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(header) => {
            if let Some(token) = header.strip_prefix("Bearer ") {
                token.trim() == admin_key
            } else {
                false
            }
        }
        None => false,
    }
}

/// GET /api/v1/todos - List all TODO items
///
/// Admin: all TODOs from all posts
/// Visitor: TODOs from published posts only
#[utoipa::path(
    get,
    path = "/api/v1/todos",
    responses(
        (status = 200, description = "List of TODO items", body = TodoListResponse)
    ),
    tag = "todos"
)]
pub async fn list_todos(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<TodoListResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let is_admin = is_admin_request(&headers);

    let mut select = todo_item::Entity::find();

    // Visitor: only TODOs from published posts
    if !is_admin {
        let published_post_ids: Vec<i32> = post::Entity::find()
            .filter(post::Column::Published.eq(true))
            .all(db)
            .await?
            .into_iter()
            .map(|p| p.id)
            .collect();
        select = select.filter(todo_item::Column::PostId.is_in(published_post_ids));
    }

    let todos = select.all(db).await?;

    // Get subscriber counts for all todos in one query
    let todo_ids: Vec<i32> = todos.iter().map(|t| t.id).collect();
    let subscriptions = if todo_ids.is_empty() {
        Vec::new()
    } else {
        todo_subscription::Entity::find()
            .filter(todo_subscription::Column::TodoItemId.is_in(todo_ids))
            .all(db)
            .await?
    };

    let mut sub_counts: HashMap<i32, i64> = HashMap::new();
    for sub in subscriptions {
        *sub_counts.entry(sub.todo_item_id).or_insert(0) += 1;
    }

    let items = todos
        .iter()
        .map(|todo| TodoResponse {
            id: todo.id,
            post_id: todo.post_id,
            title: todo.title.clone(),
            completed: todo.completed,
            subscriber_count: sub_counts.get(&todo.id).copied().unwrap_or(0),
        })
        .collect();

    Ok(Json(TodoListResponse { items }))
}

/// POST /api/v1/todos/:id/subscribe - Subscribe to a TODO item
#[utoipa::path(
    post,
    path = "/api/v1/todos/{id}/subscribe",
    params(
        ("id" = i32, Path, description = "TODO item ID"),
    ),
    request_body = SubscribeRequest,
    responses(
        (status = 201, description = "Subscribed successfully"),
        (status = 400, description = "Bad request - must provide email or user_id"),
        (status = 404, description = "TODO item not found"),
        (status = 409, description = "Already subscribed")
    ),
    tag = "todos"
)]
pub async fn subscribe_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<SubscribeRequest>,
) -> Result<StatusCode, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // Validate that at least one of email or user_id is provided
    if body.email.is_none() && body.user_id.is_none() {
        return Err(AppError::BadRequest(
            "Must provide either email or user_id".to_string(),
        ));
    }

    // Verify TODO item exists
    let _todo = todo_item::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("TODO item with id {} not found", id)))?;

    // Check for existing subscription
    let mut existing_query = todo_subscription::Entity::find()
        .filter(todo_subscription::Column::TodoItemId.eq(id));

    if let Some(ref email) = body.email {
        existing_query =
            existing_query.filter(todo_subscription::Column::Email.eq(email.as_str()));
    }
    if let Some(user_id) = body.user_id {
        existing_query = existing_query.filter(todo_subscription::Column::UserId.eq(user_id));
    }

    let existing = existing_query.one(db).await?;
    if existing.is_some() {
        return Err(AppError::Conflict("Already subscribed".to_string()));
    }

    let now = Utc::now();
    let new_subscription = todo_subscription::ActiveModel {
        todo_item_id: Set(id),
        user_id: Set(body.user_id),
        email: Set(body.email),
        created_at: Set(now),
        ..Default::default()
    };

    new_subscription.insert(db).await?;
    tracing::info!("New subscription for todo {}", id);

    Ok(StatusCode::CREATED)
}

/// DELETE /api/v1/todos/:id/subscribe - Unsubscribe from a TODO item
#[utoipa::path(
    delete,
    path = "/api/v1/todos/{id}/subscribe",
    params(
        ("id" = i32, Path, description = "TODO item ID"),
    ),
    request_body = UnsubscribeRequest,
    responses(
        (status = 204, description = "Unsubscribed successfully"),
        (status = 400, description = "Bad request - must provide email or user_id"),
        (status = 404, description = "Subscription not found")
    ),
    tag = "todos"
)]
pub async fn unsubscribe_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<UnsubscribeRequest>,
) -> Result<StatusCode, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // Validate that at least one of email or user_id is provided
    if body.email.is_none() && body.user_id.is_none() {
        return Err(AppError::BadRequest(
            "Must provide either email or user_id".to_string(),
        ));
    }

    // Find the subscription to delete
    let mut query = todo_subscription::Entity::find()
        .filter(todo_subscription::Column::TodoItemId.eq(id));

    if let Some(ref email) = body.email {
        query = query.filter(todo_subscription::Column::Email.eq(email.as_str()));
    }
    if let Some(user_id) = body.user_id {
        query = query.filter(todo_subscription::Column::UserId.eq(user_id));
    }

    let subscription = query
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound("Subscription not found".to_string()))?;

    subscription.delete(db).await?;
    tracing::info!("Unsubscribed from todo {}", id);

    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/v1/todos/:id/complete - Mark a TODO item as completed (admin only)
#[utoipa::path(
    post,
    path = "/api/v1/todos/{id}/complete",
    params(
        ("id" = i32, Path, description = "TODO item ID"),
    ),
    responses(
        (status = 200, description = "TODO marked as completed"),
        (status = 404, description = "TODO item not found")
    ),
    tag = "todos"
)]
pub async fn complete_todo(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<TodoResponse>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let todo = todo_item::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("TODO item with id {} not found", id)))?;

    let mut active_model: todo_item::ActiveModel = todo.into();
    active_model.completed = Set(true);
    active_model.updated_at = Set(Utc::now());

    let updated = active_model.update(db).await?;

    let post_title = post::Entity::find_by_id(updated.post_id)
        .one(db)
        .await?
        .map(|p| p.title)
        .unwrap_or_else(|| "Unknown Post".to_string());

    if let Err(e) = notify_todo_subscribers(db, &state.config, &updated, &post_title).await {
        tracing::error!("Failed to send TODO completion notifications: {}", e);
    }

    // Get subscriber count
    let subscriber_count = todo_subscription::Entity::find()
        .filter(todo_subscription::Column::TodoItemId.eq(id))
        .count(db)
        .await? as i64;

    Ok(Json(TodoResponse {
        id: updated.id,
        post_id: updated.post_id,
        title: updated.title,
        completed: updated.completed,
        subscriber_count,
    }))
}
