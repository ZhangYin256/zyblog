use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::Router;

use crate::handlers::videos::upload_video;

/// Video upload routes.
pub fn video_routes() -> Router {
    Router::new()
        .route("/api/v1/videos", post(upload_video))
        .layer(DefaultBodyLimit::max(105_000_000)) // 100MB + multipart overhead
}
