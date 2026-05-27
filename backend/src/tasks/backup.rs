use chrono::Utc;
use std::path::Path;
use tokio::fs;
use tokio::process::Command;

use crate::handlers::backup::BackupInfo;

pub async fn create_pg_dump(
    database_url: &str,
    backup_dir: &Path,
) -> Result<(String, u64), anyhow::Error> {
    fs::create_dir_all(backup_dir).await?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("backup_{}.sql.gz", timestamp);
    let filepath = backup_dir.join(&filename);

    let output = Command::new("pg_dump")
        .arg(database_url)
        .arg("--no-owner")
        .arg("--no-privileges")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("pg_dump failed: {}", stderr));
    }

    let mut encoder = flate2::write::GzEncoder::new(
        Vec::new(),
        flate2::Compression::default(),
    );
    std::io::Write::write_all(&mut encoder, &output.stdout)?;
    let compressed = encoder.finish()?;

    fs::write(&filepath, &compressed).await?;

    let size = compressed.len() as u64;

    tracing::info!("Backup created: {} ({} bytes)", filename, size);

    Ok((filename, size))
}

pub async fn list_backup_files(backup_dir: &Path) -> Result<Vec<BackupInfo>, anyhow::Error> {
    if !backup_dir.exists() {
        return Ok(Vec::new());
    }

    let mut entries = fs::read_dir(backup_dir).await?;
    let mut backups = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("gz") {
            let metadata = fs::metadata(&path).await?;
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string();

            let created_at = metadata
                .modified()
                .map(|t| {
                    let datetime: chrono::DateTime<Utc> = t.into();
                    datetime.to_rfc3339()
                })
                .unwrap_or_else(|_| "unknown".to_string());

            backups.push(BackupInfo {
                filename,
                size_bytes: metadata.len(),
                created_at,
            });
        }
    }

    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(backups)
}

pub async fn restore_from_dump(
    database_url: &str,
    backup_path: &Path,
) -> Result<(), anyhow::Error> {
    let compressed = fs::read(backup_path).await?;

    let mut decoder = flate2::read::GzDecoder::new(&compressed[..]);
    let mut sql = String::new();
    std::io::Read::read_to_string(&mut decoder, &mut sql)?;

    let output = Command::new("psql")
        .arg(database_url)
        .arg("-c")
        .arg(&sql)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("psql restore failed: {}", stderr));
    }

    tracing::info!("Database restored from: {:?}", backup_path);

    Ok(())
}

pub async fn cleanup_old_backups(
    backup_dir: &Path,
    max_count: usize,
) -> Result<usize, anyhow::Error> {
    let mut backups = list_backup_files(backup_dir).await?;

    if backups.len() <= max_count {
        return Ok(0);
    }

    backups.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    let to_remove = backups.len() - max_count;
    let mut removed = 0;

    for backup in backups.iter().take(to_remove) {
        let path = backup_dir.join(&backup.filename);
        if fs::remove_file(&path).await.is_ok() {
            removed += 1;
            tracing::info!("Removed old backup: {}", backup.filename);
        }
    }

    Ok(removed)
}

pub async fn run_scheduled_backup(
    database_url: &str,
    backup_dir: &Path,
    retention_count: usize,
) -> Result<(), anyhow::Error> {
    tracing::info!("Running scheduled backup...");

    let (filename, size) = create_pg_dump(database_url, backup_dir).await?;
    tracing::info!("Scheduled backup completed: {} ({} bytes)", filename, size);

    let removed = cleanup_old_backups(backup_dir, retention_count).await?;
    if removed > 0 {
        tracing::info!("Cleaned up {} old backup(s)", removed);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_list_empty_backups() {
        let tmp = TempDir::new().unwrap();
        let backups = list_backup_files(tmp.path()).await.unwrap();
        assert!(backups.is_empty());
    }

    #[tokio::test]
    async fn test_cleanup_preserves_count() {
        let tmp = TempDir::new().unwrap();
        let result = cleanup_old_backups(tmp.path(), 5).await.unwrap();
        assert_eq!(result, 0);
    }
}
