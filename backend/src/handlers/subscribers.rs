use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::error::AppError;
use crate::models::subscriber::{self, Entity as Subscriber};
use crate::state::AppState;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateSubscriberRequest {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct SubscriberResponse {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
    pub confirmed: bool,
}

impl From<subscriber::Model> for SubscriberResponse {
    fn from(m: subscriber::Model) -> Self {
        Self {
            id: m.id,
            email: m.email,
            name: m.name,
            confirmed: m.confirmed,
        }
    }
}

/// POST /api/v1/subscribers
///
/// 创建新订阅者。如果邮箱已订阅则返回 409
#[utoipa::path(
    post,
    path = "/api/v1/subscribers",
    request_body = CreateSubscriberRequest,
    responses(
        (status = 201, description = "Subscriber created", body = SubscriberResponse),
        (status = 409, description = "Email already subscribed")
    ),
    tag = "subscribers"
)]
pub async fn create_subscriber(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateSubscriberRequest>,
) -> Result<(StatusCode, Json<SubscriberResponse>), AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    // 检查重复邮箱
    let existing = Subscriber::find()
        .filter(subscriber::Column::Email.eq(&body.email))
        .one(db)
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Email already subscribed".to_string()));
    }

    let now = chrono::Utc::now();
    let new_subscriber = subscriber::ActiveModel {
        email: Set(body.email),
        name: Set(body.name),
        confirmed: Set(false),
        created_at: Set(now),
        ..Default::default()
    };

    let model = new_subscriber.insert(db).await?;
    tracing::info!("New subscriber: {}", model.email);

    Ok((StatusCode::CREATED, Json(SubscriberResponse::from(model))))
}

/// GET /api/v1/subscribers
///
/// 列出所有订阅者（仅管理员）
#[utoipa::path(
    get,
    path = "/api/v1/subscribers",
    responses(
        (status = 200, description = "List of subscribers", body = Vec<SubscriberResponse>)
    ),
    tag = "subscribers"
)]
pub async fn list_subscribers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SubscriberResponse>>, AppError> {
    let db = state.db.as_ref().ok_or_else(|| {
        AppError::Internal(anyhow::anyhow!("Database not available"))
    })?;

    let subscribers = Subscriber::find().all(db).await?;
    let response: Vec<SubscriberResponse> = subscribers
        .into_iter()
        .map(SubscriberResponse::from)
        .collect();

    Ok(Json(response))
}
