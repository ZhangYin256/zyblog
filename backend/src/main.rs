mod config;
mod error;
mod handlers;
mod middleware;
mod migrations;
mod models;
mod routes;
mod state;
mod storage;
mod tasks;
mod utils;

use axum::{routing::{delete, get, post, put}, Json, Router};
use axum::extract::DefaultBodyLimit;
use config::Config;
use handlers::{posts, pulls, revisions, images, videos, tags};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use sea_orm_migration::MigratorTrait;
use routes::backup::backup_routes;
use routes::export::export_routes;
use routes::subscribers::routes as subscriber_routes;
use routes::auth::routes as auth_routes;
use routes::tags::tag_routes;
use routes::media::media_routes;
use routes::revisions::revision_routes;
use routes::comments::{post_comments_routes, comments_routes};
use routes::todos::todo_routes;
use serde_json::{json, Value};
use state::AppState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;

/// 健康检查端点
async fn health_check() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn openapi_json() -> Json<Value> {
    Json(ApiDoc::openapi().to_json().map_or_else(
        |_| json!({"error": "Failed to generate OpenAPI spec"}),
        |spec| serde_json::from_str(&spec).unwrap_or(json!({"error": "Invalid spec"})),
    ))
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "ZYBlog API",
        description = "REST API for ZYBlog, a modern blog platform.",
        version = "0.1.0"
    ),
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::auth::refresh,
        handlers::auth::me,
        handlers::auth::update_profile,
        handlers::auth::change_password,
        handlers::auth::github_login,
        handlers::auth::github_callback,
        handlers::auth::github_callback_json,
        handlers::posts::create_post,
        handlers::posts::list_posts,
        handlers::posts::get_post,
        handlers::posts::update_post,
        handlers::posts::delete_post,
        handlers::posts::get_post_todos,
        handlers::posts::trash_posts,
        handlers::posts::restore_post,
        handlers::posts::permanent_delete_post,
        handlers::subscribers::create_subscriber,
        handlers::subscribers::list_subscribers,
        handlers::export::export_posts,
        handlers::pulls::create_pull,
        handlers::pulls::list_pulls,
        handlers::pulls::update_pull,
        handlers::pulls::apply_pull,
        handlers::pulls::add_comment,
        handlers::pulls::list_comments,
        handlers::tags::create_tag,
        handlers::tags::list_tags,
        handlers::tags::delete_tag,
        handlers::tags::assign_tags_to_post,
        handlers::tags::remove_tag_from_post,
        handlers::tags::list_post_tags,
        handlers::revisions::create_revision,
        handlers::revisions::list_revisions,
        handlers::revisions::get_revision,
        handlers::revisions::rollback_revision,
        handlers::revisions::diff_revisions,
        handlers::media::upload_media,
        handlers::media::list_media,
        handlers::media::delete_media,
        handlers::todos::list_todos,
        handlers::todos::subscribe_todo,
        handlers::todos::unsubscribe_todo,
        handlers::todos::complete_todo,
    ),
    components(schemas(
        handlers::auth::RegisterRequest,
        handlers::auth::LoginRequest,
        handlers::auth::RefreshRequest,
        handlers::auth::UserResponse,
        handlers::auth::AuthResponse,
        handlers::auth::RefreshResponse,
        handlers::auth::MeResponse,
        handlers::auth::UpdateProfileRequest,
        handlers::auth::ChangePasswordRequest,
        handlers::auth::SuccessResponse,
        handlers::auth::TokenResponse,
        handlers::posts::PostResponse,
        handlers::posts::PostListResponse,
        handlers::posts::CreatePostRequest,
        handlers::posts::CreatePostResponse,
        handlers::posts::UpdatePostRequest,
        handlers::posts::TodoItemResponse,
        handlers::subscribers::SubscriberResponse,
        handlers::subscribers::CreateSubscriberRequest,
        handlers::export::ExportedPost,
        handlers::pulls::PullResponse,
        handlers::pulls::PullListResponse,
        handlers::pulls::CreatePullRequest,
        handlers::pulls::UpdatePullRequest,
        handlers::pulls::ApplyPullResponse,
        handlers::pulls::AddCommentRequest,
        handlers::pulls::CommentResponse,
        handlers::pulls::CommentListResponse,
        crate::utils::Fragment,
        handlers::tags::TagResponse,
        handlers::tags::CreateTagRequest,
        handlers::tags::AssignTagsRequest,
        handlers::revisions::RevisionResponse,
        handlers::revisions::RevisionListResponse,
        handlers::revisions::DiffResponse,
        handlers::media::MediaResponse,
        handlers::media::MediaListResponse,
        handlers::todos::TodoResponse,
        handlers::todos::TodoListResponse,
        handlers::todos::SubscribeRequest,
        handlers::todos::UnsubscribeRequest,
    )),
    tags(
        (name = "auth", description = "Authentication"),
        (name = "posts", description = "Blog post management"),
        (name = "subscribers", description = "Newsletter subscribers"),
        (name = "export", description = "Data export"),
        (name = "pulls", description = "Pull request interactions"),
        (name = "tags", description = "Tag management"),
        (name = "revisions", description = "Post revision management"),
        (name = "media", description = "Media file management"),
        (name = "todos", description = "TODO item management"),
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "migrate" {
        let config = Config::from_env()?;
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                EnvFilter::new("info")
            }))
            .init();

        let db = sea_orm::Database::connect(&config.database_url).await?;
        tracing::info!("Running database migrations...");
        migrations::Migrator::up(&db, None).await?;
        tracing::info!("Migrations completed successfully!");
        return Ok(());
    }

    let config = Config::from_env()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(&config.rust_log)
        }))
        .init();

    tracing::info!("Starting zyblog server on {}", config.server_addr);

    let db = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        sea_orm::Database::connect(&config.database_url),
    )
    .await
    .ok()
    .and_then(|r| r.ok());

    if db.is_some() {
        tracing::info!("Database connected");
    } else {
        tracing::warn!("Database not available, running without DB");
    }

    let state = Arc::new(AppState {
        db,
        config: config.clone(),
    });

    if let Some(db) = &state.db {
        if !state.config.admin_key.is_empty() {
            let admin = models::user::Entity::find()
                .filter(models::user::Column::Role.eq("admin"))
                .one(db)
                .await;

            match admin {
                Ok(None) => {
                    let default_password = state.config.admin_key.clone();
                    let password_hash = bcrypt::hash(&default_password, bcrypt::DEFAULT_COST)
                        .unwrap_or_default();

                    let admin_user = models::user::ActiveModel {
                        email: Set("admin@zyblog.local".to_string()),
                        name: Set("Admin".to_string()),
                        role: Set("admin".to_string()),
                        password_hash: Set(Some(password_hash)),
                        ..Default::default()
                    };
                    match admin_user.insert(db).await {
                        Ok(_) => tracing::info!("Admin user created with default password: {}", default_password),
                        Err(e) => tracing::error!("Failed to create admin user: {}", e),
                    }
                }
                Ok(Some(_)) => tracing::debug!("Admin user already exists"),
                Err(e) => tracing::error!("Failed to query admin user: {}", e),
            }
        }
    }

    // Start scheduled backup task
    let backup_state = state.clone();
    tokio::spawn(async move {
        let backup_dir = std::path::PathBuf::from(&backup_state.config.backup_dir);
        let interval_hours = backup_state.config.backup_interval_hours;
        let retention = backup_state.config.backup_retention_count;

        tracing::info!(
            "Backup scheduler started: interval={}h, retention={}",
            interval_hours,
            retention
        );

        loop {
            tokio::time::sleep(std::time::Duration::from_secs(interval_hours * 3600)).await;

            if let Err(e) = tasks::backup::run_scheduled_backup(
                &backup_state.config.database_url,
                &backup_dir,
                retention,
            )
            .await
            {
                tracing::error!("Scheduled backup failed: {}", e);
            }
        }
    });

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/v1/posts", get(posts::list_posts).post(posts::create_post))
        .route("/api/v1/posts/search", get(posts::search_posts))
        .route("/api/v1/posts/trash", get(posts::trash_posts))
        .route("/api/v1/posts/:id", get(posts::get_post).put(posts::update_post).delete(posts::delete_post))
        .route("/api/v1/posts/:id/todos", get(posts::get_post_todos))
        .route("/api/v1/posts/:id/restore", post(posts::restore_post))
        .route("/api/v1/posts/:id/permanent", delete(posts::permanent_delete_post))
        .route("/api/v1/posts/:id/diff", get(revisions::diff_revisions))
        .route("/api/v1/posts/:id/pulls", get(pulls::list_pulls).post(pulls::create_pull))
        .route("/api/v1/posts/:id/tags", get(tags::list_post_tags).post(tags::assign_tags_to_post))
        .route("/api/v1/posts/:id/tags/:tag_id", delete(tags::remove_tag_from_post))
        .nest(
            "/api/v1/posts/:id/revisions",
            revision_routes().with_state(state.clone()),
        )
        .route("/api/v1/pulls/:id", put(pulls::update_pull))
        .route("/api/v1/pulls/:id/apply", post(pulls::apply_pull))
        .route("/api/v1/pulls/:id/comments", post(pulls::add_comment).get(pulls::list_comments))
        .route("/api/v1/images", post(images::upload_image))
        .route("/api/v1/videos", post(videos::upload_video))
        .with_state(state.clone())
        .nest(
            "/api/v1/posts/:id/comments",
            post_comments_routes().with_state(state.clone()),
        )
        .nest(
            "/api/v1/comments",
            comments_routes().with_state(state.clone()),
        )
        .nest(
            "/api/v1/subscribers",
            subscriber_routes().with_state(state.clone()),
        )
        .nest("/api/v1/tags", tag_routes().with_state(state.clone()))
        .nest("/api/v1/export", export_routes().with_state(state.clone()))
        .nest("/api/v1/backup", backup_routes().with_state(state.clone()))
        .nest("/api/v1/auth", auth_routes().with_state(state.clone()))
        .nest("/api/v1/media", media_routes().with_state(state.clone()))
        .nest("/api/v1/todos", todo_routes().with_state(state.clone()))
        .route("/api-docs/openapi.json", get(openapi_json))
        .route("/swagger-ui/", get(|| async {
            axum::response::Redirect::permanent("/static/swagger-ui.html")
        }))
        .nest_service("/static", ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(105_000_000))
        .layer(axum::middleware::from_fn_with_state(state.clone(), middleware::auth::admin_auth_middleware));

    let listener = TcpListener::bind(&config.server_addr).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
