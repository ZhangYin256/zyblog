use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers::{posts, pulls};
use crate::state::AppState;

/// Combined posts routes (public + protected).
///
/// The auth middleware (applied in main.rs) already passes GET/HEAD/OPTIONS
/// without requiring authentication, so all routes can live in one router.
pub fn posts_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Public (read) routes
        .route("/", get(posts::list_posts))
        .route("/trash", get(posts::trash_posts))
        .route("/{id}", get(posts::get_post))
        .route("/{id}/todos", get(posts::get_post_todos))
        .route("/{id}/pulls", get(pulls::list_pulls).post(pulls::create_pull))
        // Protected (write) routes
        .route("/", post(posts::create_post))
        .route("/{id}", put(posts::update_post).delete(posts::delete_post))
        .route("/{id}/restore", post(posts::restore_post))
        .route("/{id}/permanent", delete(posts::permanent_delete_post))
}
