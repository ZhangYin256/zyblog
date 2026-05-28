use axum::{routing::{delete, get, post}, Router};
use std::sync::Arc;

use crate::handlers::backup;
use crate::state::AppState;

pub fn backup_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(backup::create_backup))
        .route("/list", get(backup::list_backups))
        .route("/restore", post(backup::restore_backup))
        .route("/:filename", delete(backup::delete_backup))
}
