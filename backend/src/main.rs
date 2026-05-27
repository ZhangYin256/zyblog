mod config;
mod error;
mod handlers;
mod middleware;
mod migrations;
mod models;
mod routes;
mod state;
mod tasks;

use axum::{routing::get, Json, Router};
use config::Config;
use routes::export::export_routes;
use routes::images::image_routes;
use routes::posts::posts_routes;
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
    )),
    tags(
        (name = "posts", description = "Blog post management"),
        (name = "subscribers", description = "Newsletter subscribers"),
        (name = "export", description = "Data export")
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    // 公开路由（无需认证）
    let public_routes = Router::new()
        .route("/api/health", get(health_check))
        .merge(image_routes())
        .nest(
            "/api/v1/subscribers",
            subscriber_routes().with_state(state.clone()),
        );

    // 受保护路由（写操作需要认证）
    let protected_routes = Router::new()
        .nest("/api/v1/posts", posts_routes().with_state(state.clone()))
        .nest("/api/v1/export", export_routes().with_state(state.clone()))
        .layer(axum::middleware::from_fn(middleware::auth::admin_auth_middleware));

    let app = public_routes
        .merge(protected_routes)
        .route("/api-docs/openapi.json", get(openapi_json))
        .route("/swagger-ui/", get(|| async {
            axum::response::Redirect::permanent("/static/swagger-ui.html")
        }))
        .nest_service("/static", ServeDir::new("static"));

    let listener = TcpListener::bind(&config.server_addr).await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
