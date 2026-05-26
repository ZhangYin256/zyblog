use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::Router;

use crate::handlers::images::upload_image;

/// Image upload routes.
pub fn image_routes() -> Router {
    Router::new()
        .route("/api/v1/images", post(upload_image))
        .layer(DefaultBodyLimit::max(5_500_000)) // 5MB + multipart overhead
}
