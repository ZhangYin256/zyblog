use sea_orm::DatabaseConnection;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Option<DatabaseConnection>,
    pub config: Config,
}
