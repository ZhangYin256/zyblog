use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers::posts;
use crate::state::AppState;

/// Create the posts router with all CRUD routes.
pub fn posts_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(posts::create_post))
        .route("/", get(posts::list_posts))
        .route("/{id}", get(posts::get_post))
        .route("/{id}", put(posts::update_post))
        .route("/{id}", delete(posts::delete_post))
        .route("/{id}/todos", get(posts::get_post_todos))
}
