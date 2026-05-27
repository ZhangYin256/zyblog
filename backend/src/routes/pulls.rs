use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;

use crate::handlers::pulls;
use crate::state::AppState;

/// Create the pulls router.
///
/// Nested at two levels in `main.rs`:
/// - `/api/v1/posts/:id/pulls` — create & list PRs for a post
/// - `/api/v1/pulls/:id` — update PR status
/// - `/api/v1/pulls/:id/comments` — add comment
pub fn pulls_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(pulls::create_pull))
        .route("/", get(pulls::list_pulls))
}

pub fn pull_item_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", put(pulls::update_pull))
        .route("/comments", post(pulls::add_comment))
}
