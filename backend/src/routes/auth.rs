use axum::{routing::{get, post, put}, Router};
use std::sync::Arc;

use crate::handlers::auth;
use crate::state::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(auth::register))
        .route("/login", post(auth::login))
        .route("/refresh", post(auth::refresh))
        .route("/me", get(auth::me))
        .route("/profile", put(auth::update_profile))
        .route("/password", put(auth::change_password))
        .route("/github", get(auth::github_login))
        .route("/github/callback", get(auth::github_callback))
        .route("/github/token", get(auth::github_callback_json))
}
