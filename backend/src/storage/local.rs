use std::path::PathBuf;
use async_trait::async_trait;
use anyhow::Result;
use super::StorageAdapter;

/// Local filesystem storage implementation.
pub struct LocalStorage {
    base_path: PathBuf,
    base_url: String,
}

impl LocalStorage {
    pub fn new(base_path: PathBuf, base_url: String) -> Self {
        Self { base_path, base_url }
    }
}

#[async_trait]
impl StorageAdapter for LocalStorage {
    async fn upload(&self, key: &str, data: &[u8], _content_type: &str) -> Result<String> {
        let path = self.base_path.join(key);
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(&path, data).await?;
        Ok(self.get_url(key))
    }

    async fn delete(&self, key: &str) -> Result<()> {
        let path = self.base_path.join(key);
        tokio::fs::remove_file(&path).await?;
        Ok(())
    }

    fn get_url(&self, key: &str) -> String {
        format!("{}/{}", self.base_url, key)
    }
}
