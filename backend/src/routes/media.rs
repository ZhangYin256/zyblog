use axum::{
    routing::{delete, post},
    Router,
};
use std::sync::Arc;

use crate::handlers::media;
use crate::state::AppState;

/// Build media CRUD routes: /api/v1/media
pub fn media_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(media::upload_media).get(media::list_media))
        .route("/:id", delete(media::delete_media))
}
