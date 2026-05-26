use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TodoItems::Table)
                    .add_column(ColumnDef::new(TodoItems::PostId).integer().not_null().default(0))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_todo_items_post_id")
                    .table(TodoItems::Table)
                    .col(TodoItems::PostId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TodoItems::Table)
                    .drop_column(TodoItems::PostId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum TodoItems {
    Table,
    PostId,
}
