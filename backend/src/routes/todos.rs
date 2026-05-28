use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers::todos;
use crate::state::AppState;

/// Build TODO routes.
pub fn todo_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(todos::list_todos))
        .route("/{id}/subscribe", post(todos::subscribe_todo).delete(todos::unsubscribe_todo))
        .route("/{id}/complete", post(todos::complete_todo))
}
