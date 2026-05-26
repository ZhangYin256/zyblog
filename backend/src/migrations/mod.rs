pub use sea_orm_migration::prelude::*;

mod m20240101_000001_create_posts_table;
mod m20240101_000002_create_todo_items_table;
mod m20240101_000003_create_subscribers_table;
mod m20240101_000004_create_images_table;
mod m20240101_000005_add_post_id_to_todo_items;

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
        ]
    }
}
