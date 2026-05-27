use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers::posts;
use crate::state::AppState;

/// Public posts routes (GET only, no auth required).
pub fn public_posts_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(posts::list_posts))
        .route("/{id}", get(posts::get_post))
        .route("/{id}/todos", get(posts::get_post_todos))
}

/// Protected posts routes (write operations, auth required).
pub fn protected_posts_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(posts::create_post))
        .route("/{id}", put(posts::update_post))
        .route("/{id}", delete(posts::delete_post))
}
