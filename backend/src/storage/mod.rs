pub mod local;

use async_trait::async_trait;
use anyhow::Result;

/// Pluggable storage backend trait for media uploads.
#[async_trait]
pub trait StorageAdapter: Send + Sync {
    /// Upload data with the given key and content type.
    /// Returns the URL to access the uploaded file.
    async fn upload(&self, key: &str, data: &[u8], content_type: &str) -> Result<String>;

    /// Delete the file at the given key.
    async fn delete(&self, key: &str) -> Result<()>;

    /// Get the public URL for a file at the given key.
    fn get_url(&self, key: &str) -> String;
}
