use axum::{routing::{get, post}, Router};
use std::sync::Arc;

use crate::handlers::subscribers;
use crate::state::AppState;

/// Build subscriber routes.
pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(subscribers::create_subscriber))
        .route("/", get(subscribers::list_subscribers))
}
