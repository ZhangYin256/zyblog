use axum::{
    routing::{delete, get},
    Router,
};
use std::sync::Arc;

use crate::handlers::tags;
use crate::state::AppState;

/// Build tag CRUD routes: /api/v1/tags
pub fn tag_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(tags::list_tags).post(tags::create_tag))
        .route("/:id", delete(tags::delete_tag))
}
