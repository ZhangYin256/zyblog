use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::models::subscriber::{self, Entity as Subscriber};
use crate::state::AppState;

/// 向所有已确认的订阅者发送文章更新邮件通知
pub async fn notify_subscribers_on_update(
    state: &Arc<AppState>,
    post_id: i32,
    post_title: &str,
) -> Result<(), anyhow::Error> {
    let db = state.db.as_ref().ok_or_else(|| {
        anyhow::anyhow!("Database not available")
    })?;

    // 获取所有已确认的订阅者
    let subscribers = Subscriber::find()
        .filter(subscriber::Column::Confirmed.eq(true))
        .all(db)
        .await?;

    if subscribers.is_empty() {
        tracing::info!("No confirmed subscribers to notify");
        return Ok(());
    }

    let config = &state.config;

    // 构建 SMTP 传输
    let credentials = Credentials::new(
        config.smtp_username.clone(),
        config.smtp_password.clone(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)?
        .port(config.smtp_port)
        .credentials(credentials)
        .build();

    let subject = format!("ZYBlog Update: {}", post_title);
    let body_text = format!(
        "Hello,\n\nA new blog post has been published on ZYBlog:\n\n\"{}\" (ID: {})\n\nCheck it out at the blog!\n\n-- ZYBlog",
        post_title, post_id
    );

    let mut sent_count = 0;
    let mut fail_count = 0;

    for sub in &subscribers {
        let email = Message::builder()
            .from(config.smtp_from.parse()?)
            .to(sub.email.parse()?)
            .subject(&subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body_text.clone())?;

        match mailer.send(email).await {
            Ok(_) => {
                sent_count += 1;
                tracing::info!("Notification sent to {}", sub.email);
            }
            Err(e) => {
                fail_count += 1;
                tracing::error!("Failed to send notification to {}: {}", sub.email, e);
            }
        }
    }

    tracing::info!(
        "Subscriber notifications: {} sent, {} failed (post_id={})",
        sent_count,
        fail_count,
        post_id
    );

    Ok(())
}
