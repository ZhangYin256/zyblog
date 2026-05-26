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

        Ok(Self {
            server_addr,
            database_url,
            rust_log,
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_from,
        })
    }
}
