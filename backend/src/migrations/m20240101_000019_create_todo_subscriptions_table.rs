use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TodoSubscriptions::Table)
                    .col(
                        ColumnDef::new(TodoSubscriptions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TodoSubscriptions::TodoItemId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(TodoSubscriptions::UserId).integer())
                    .col(ColumnDef::new(TodoSubscriptions::Email).string_len(255))
                    .col(
                        ColumnDef::new(TodoSubscriptions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_todo_subscriptions_todo_item_id")
                            .from(TodoSubscriptions::Table, TodoSubscriptions::TodoItemId)
                            .to(TodoItems::Table, TodoItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_todo_subscriptions_user_id")
                            .from(TodoSubscriptions::Table, TodoSubscriptions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Create unique constraint on (todo_item_id, user_id)
        manager
            .create_index(
                Index::create()
                    .name("uq_todo_subscriptions_todo_item_user")
                    .table(TodoSubscriptions::Table)
                    .col(TodoSubscriptions::TodoItemId)
                    .col(TodoSubscriptions::UserId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create unique constraint on (todo_item_id, email)
        manager
            .create_index(
                Index::create()
                    .name("uq_todo_subscriptions_todo_item_email")
                    .table(TodoSubscriptions::Table)
                    .col(TodoSubscriptions::TodoItemId)
                    .col(TodoSubscriptions::Email)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Create index on todo_item_id
        manager
            .create_index(
                Index::create()
                    .name("idx_todo_subscriptions_todo_item_id")
                    .table(TodoSubscriptions::Table)
                    .col(TodoSubscriptions::TodoItemId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoSubscriptions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TodoSubscriptions {
    Table,
    Id,
    TodoItemId,
    UserId,
    Email,
    CreatedAt,
}

#[derive(DeriveIden)]
enum TodoItems {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
