pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_posts_table;
mod m20240101_000002_create_todo_items_table;
mod m20240101_000003_create_subscribers_table;
mod m20240101_000004_create_images_table;
mod m20240101_000005_add_post_id_to_todo_items;
mod m20240101_000006_create_pull_requests_table;
mod m20240101_000007_create_pull_request_comments_table;
mod m20240101_000008_create_tags_table;
mod m20240101_000009_create_post_tags_table;
mod m20240101_000010_create_comments_table;
mod m20240101_000011_create_users_table;
mod m20240101_000012_create_refresh_tokens_table;
mod m20240101_000013_add_author_id_to_posts;
mod m20240101_000014_create_post_revisions_table;
mod m20240101_000015_add_version_to_posts;
mod m20240101_000016_redesign_pull_requests_table;
mod m20240101_000017_create_media_table;
mod m20240101_000018_create_pr_comments_table;
mod m20240101_000019_create_todo_subscriptions_table;
mod m20240101_000020_add_referenced_content_to_comments;
mod m20240101_000021_add_deleted_at_to_posts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_posts_table::Migration),
            Box::new(m20240101_000002_create_todo_items_table::Migration),
            Box::new(m20240101_000003_create_subscribers_table::Migration),
            Box::new(m20240101_000004_create_images_table::Migration),
            Box::new(m20240101_000005_add_post_id_to_todo_items::Migration),
            Box::new(m20240101_000006_create_pull_requests_table::Migration),
            Box::new(m20240101_000007_create_pull_request_comments_table::Migration),
            Box::new(m20240101_000008_create_tags_table::Migration),
            Box::new(m20240101_000009_create_post_tags_table::Migration),
            Box::new(m20240101_000010_create_comments_table::Migration),
            Box::new(m20240101_000011_create_users_table::Migration),
            Box::new(m20240101_000012_create_refresh_tokens_table::Migration),
            Box::new(m20240101_000013_add_author_id_to_posts::Migration),
            Box::new(m20240101_000014_create_post_revisions_table::Migration),
            Box::new(m20240101_000015_add_version_to_posts::Migration),
            Box::new(m20240101_000016_redesign_pull_requests_table::Migration),
            Box::new(m20240101_000017_create_media_table::Migration),
            Box::new(m20240101_000018_create_pr_comments_table::Migration),
            Box::new(m20240101_000019_create_todo_subscriptions_table::Migration),
            Box::new(m20240101_000020_add_referenced_content_to_comments::Migration),
            Box::new(m20240101_000021_add_deleted_at_to_posts::Migration),
        ]
    }
}
