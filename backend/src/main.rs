mod config;
mod error;
mod handlers;
mod middleware;
mod migrations;
mod models;
mod routes;
mod state;
mod tasks;

use axum::{routing::{get, post, put}, Json, Router};
use axum::extract::DefaultBodyLimit;
use config::Config;
use handlers::{posts, pulls, images, videos};
use sea_orm_migration::MigratorTrait;
use routes::backup::backup_routes;
use routes::export::export_routes;
use routes::subscribers::routes as subscriber_routes;
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
        handlers::posts::create_post,
        handlers::posts::list_posts,
        handlers::posts::get_post,
        handlers::posts::update_post,
        handlers::posts::delete_post,
        handlers::posts::get_post_todos,
        handlers::subscribers::create_subscriber,
        handlers::subscribers::list_subscribers,
        handlers::export::export_posts,
        handlers::pulls::create_pull,
        handlers::pulls::list_pulls,
        handlers::pulls::update_pull,
        handlers::pulls::add_comment,
    ),
    components(schemas(
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
        handlers::pulls::AddCommentRequest,
        handlers::pulls::CommentResponse,
    )),
    tags(
        (name = "posts", description = "Blog post management"),
        (name = "subscribers", description = "Newsletter subscribers"),
        (name = "export", description = "Data export"),
        (name = "pulls", description = "Pull request interactions")
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
        .route("/api/v1/posts/:id", get(posts::get_post).put(posts::update_post).delete(posts::delete_post))
        .route("/api/v1/posts/:id/todos", get(posts::get_post_todos))
        .route("/api/v1/posts/:id/pulls", get(pulls::list_pulls).post(pulls::create_pull))
        .route("/api/v1/pulls/:id", put(pulls::update_pull))
        .route("/api/v1/pulls/:id/comments", post(pulls::add_comment))
        .route("/api/v1/images", post(images::upload_image))
        .route("/api/v1/videos", post(videos::upload_video))
        .with_state(state.clone())
        .nest(
            "/api/v1/subscribers",
            subscriber_routes().with_state(state.clone()),
        )
        .nest("/api/v1/export", export_routes().with_state(state.clone()))
        .nest("/api/v1/backup", backup_routes().with_state(state.clone()))
        .route("/api-docs/openapi.json", get(openapi_json))
        .route("/swagger-ui/", get(|| async {
            axum::response::Redirect::permanent("/static/swagger-ui.html")
        }))
        .nest_service("/static", ServeDir::new("static"))
        .layer(DefaultBodyLimit::max(105_000_000))
        .layer(axum::middleware::from_fn(middleware::auth::admin_auth_middleware));

    let listener = TcpListener::bind(&config.server_addr).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
