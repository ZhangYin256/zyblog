use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers::comments;
use crate::state::AppState;

/// Create the post-level comments router.
///
/// Nested at `/api/v1/posts/:id/comments` in `main.rs`:
/// - GET  /  — list approved comments (public)
/// - POST /  — submit comment (public)
pub fn post_comments_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(comments::list_approved_comments))
        .route("/", post(comments::create_comment))
}

/// Create the top-level comments router.
///
/// Nested at `/api/v1/comments` in `main.rs`:
/// - GET  /pending     — list all pending comments (auth)
/// - PUT  /:id/approve — approve a comment (auth)
/// - DELETE /:id       — delete a comment (auth)
pub fn comments_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/pending", get(comments::list_pending_comments))
        .route("/:id/approve", put(comments::approve_comment))
        .route("/:id", delete(comments::delete_comment))
}
