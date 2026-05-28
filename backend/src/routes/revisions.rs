use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers::revisions;
use crate::state::AppState;

/// Create the revisions router.
///
/// Nested at `/api/v1/posts/:id/revisions` in `main.rs`.
pub fn revision_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(revisions::create_revision).get(revisions::list_revisions))
        .route("/:rev_id", get(revisions::get_revision))
        .route("/:rev_id/rollback", post(revisions::rollback_revision))
}
