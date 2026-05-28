use anyhow::Result;
use std::env;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Server listen address (e.g., "0.0.0.0:8080")
    pub server_addr: String,
    /// Database connection URL
    pub database_url: String,
    /// Rust log level (e.g., "info", "debug")
    pub rust_log: String,
    /// SMTP server host
    pub smtp_host: String,
    /// SMTP server port
    pub smtp_port: u16,
    /// SMTP username
    pub smtp_username: String,
    /// SMTP password
    pub smtp_password: String,
    /// Email "from" address
    pub smtp_from: String,
    /// Backup directory path
    pub backup_dir: String,
    /// Backup interval in hours
    pub backup_interval_hours: u64,
    /// Maximum number of backup files to retain
    pub backup_retention_count: usize,
    /// JWT signing secret
    pub jwt_secret: String,
    /// JWT access token expiry in seconds (default 900 = 15 min)
    pub jwt_access_expiry: u64,
    /// JWT refresh token expiry in seconds (default 604800 = 7 days)
    pub jwt_refresh_expiry: u64,
    /// GitHub OAuth client ID
    pub github_client_id: String,
    /// GitHub OAuth client secret
    pub github_client_secret: String,
    /// GitHub OAuth redirect URI
    pub github_redirect_uri: String,
    /// Admin API key for backward-compatible Bearer token auth
    pub admin_key: String,
}

impl Config {
    /// Load configuration from environment variables (with .env fallback).
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/zyblog".to_string());
        let rust_log =
            env::var("RUST_LOG").unwrap_or_else(|_| "zyblog=debug,tower_http=debug".to_string());

        let smtp_host = env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".to_string());
        let smtp_port: u16 = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse()
            .unwrap_or(587);
        let smtp_username = env::var("SMTP_USERNAME").unwrap_or_default();
        let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_default();
        let smtp_from =
            env::var("SMTP_FROM").unwrap_or_else(|_| "noreply@zyblog.local".to_string());

        let backup_dir =
            env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
        let backup_interval_hours: u64 = env::var("BACKUP_INTERVAL_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .unwrap_or(24);
        let backup_retention_count: usize = env::var("BACKUP_RETENTION_COUNT")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".to_string());
        let jwt_access_expiry: u64 = env::var("JWT_ACCESS_EXPIRY")
            .unwrap_or_else(|_| "900".to_string())
            .parse()
            .unwrap_or(900);
        let jwt_refresh_expiry: u64 = env::var("JWT_REFRESH_EXPIRY")
            .unwrap_or_else(|_| "604800".to_string())
            .parse()
            .unwrap_or(604800);
        let github_client_id = env::var("GITHUB_CLIENT_ID").unwrap_or_default();
        let github_client_secret = env::var("GITHUB_CLIENT_SECRET").unwrap_or_default();
        let github_redirect_uri =
            env::var("GITHUB_REDIRECT_URI").unwrap_or_else(|_| "http://localhost:8080/api/v1/auth/github/callback".to_string());

        let admin_key = env::var("ADMIN_KEY").unwrap_or_default();

        Ok(Self {
            server_addr,
            database_url,
            rust_log,
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_from,
            backup_dir,
            backup_interval_hours,
            backup_retention_count,
            jwt_secret,
            jwt_access_expiry,
            jwt_refresh_expiry,
            github_client_id,
            github_client_secret,
            github_redirect_uri,
            admin_key,
        })
    }
}
