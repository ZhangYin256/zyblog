use axum::{routing::get, Router};
use std::sync::Arc;

use crate::handlers::export;
use crate::state::AppState;

/// Create the export router.
pub fn export_routes() -> Router<Arc<AppState>> {
    Router::new().route("/posts", get(export::export_posts))
}
