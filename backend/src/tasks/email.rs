use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::config::Config;
use crate::error::AppError;
use crate::models::subscriber::{self, Entity as Subscriber};
use crate::models::{todo_item, todo_subscription, user};
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

/// 向单个订阅者发送 TODO 完成通知邮件
async fn send_todo_completion_email(
    config: &Config,
    to: &str,
    todo_title: &str,
    post_title: &str,
) -> Result<(), AppError> {
    let credentials = Credentials::new(
        config.smtp_username.clone(),
        config.smtp_password.clone(),
    );

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
        .map_err(|e| anyhow::anyhow!(e))?
        .port(config.smtp_port)
        .credentials(credentials)
        .build();

    let subject = format!("TODO 完成通知: {}", todo_title);
    let body_html = format!(
        r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"></head>
<body style="font-family: sans-serif; padding: 20px;">
  <h2 style="color: #333;">TODO 完成通知</h2>
  <p>您好，</p>
  <p>您关注的 TODO 事项已经完成：</p>
  <blockquote style="border-left: 4px solid #4CAF50; padding: 10px; background: #f9f9f9; margin: 10px 0;">
    <strong>{}</strong>
  </blockquote>
  <p>所属文章：<strong>{}</strong></p>
  <p>感谢您的关注！</p>
  <hr style="border: none; border-top: 1px solid #eee; margin: 20px 0;">
  <p style="color: #999; font-size: 12px;">此邮件由 ZYBlog 自动发送</p>
</body>
</html>"#,
        todo_title, post_title
    );

    let from_addr: lettre::Address = config.smtp_from.parse().map_err(|e: lettre::address::AddressError| anyhow::anyhow!(e))?;
    let to_addr: lettre::Address = to.parse().map_err(|e: lettre::address::AddressError| anyhow::anyhow!(e))?;

    let email = Message::builder()
        .from(from_addr.into())
        .to(to_addr.into())
        .subject(&subject)
        .header(ContentType::TEXT_HTML)
        .body(body_html)
        .map_err(|e| anyhow::anyhow!(e))?;

    mailer.send(email).await.map_err(|e| anyhow::anyhow!(e))?;
    tracing::info!("TODO completion notification sent to {}", to);

    Ok(())
}

/// 向所有订阅者发送 TODO 完成通知
pub async fn notify_todo_subscribers(
    db: &DatabaseConnection,
    config: &Config,
    todo_item: &todo_item::Model,
    post_title: &str,
) -> Result<(), AppError> {
    let subscriptions = todo_subscription::Entity::find()
        .filter(todo_subscription::Column::TodoItemId.eq(todo_item.id))
        .all(db)
        .await?;

    if subscriptions.is_empty() {
        tracing::info!("No subscribers for todo {}", todo_item.id);
        return Ok(());
    }

    let mut sent_count = 0;
    let mut fail_count = 0;

    for sub in subscriptions {
        let email = match sub.email {
            Some(ref e) => Some(e.clone()),
            None => {
                if let Some(uid) = sub.user_id {
                    user::Entity::find_by_id(uid)
                        .one(db)
                        .await?
                        .map(|u| u.email)
                } else {
                    None
                }
            }
        };

        if let Some(to) = email {
            match send_todo_completion_email(config, &to, &todo_item.title, post_title).await {
                Ok(_) => sent_count += 1,
                Err(e) => {
                    fail_count += 1;
                    tracing::error!(
                        "Failed to send TODO completion notification to {}: {}",
                        to,
                        e
                    );
                }
            }
        } else {
            tracing::warn!(
                "Subscription {} has no email and no valid user_id, skipping",
                sub.id
            );
        }
    }

    tracing::info!(
        "TODO completion notifications: {} sent, {} failed (todo_id={})",
        sent_count,
        fail_count,
        todo_item.id
    );

    Ok(())
}
