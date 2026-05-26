use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TodoItems::Table)
                    .col(
                        ColumnDef::new(TodoItems::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TodoItems::Title).string().not_null())
                    .col(ColumnDef::new(TodoItems::Description).text())
                    .col(ColumnDef::new(TodoItems::Completed).boolean().not_null().default(false))
                    .col(
                        ColumnDef::new(TodoItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(TodoItems::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TodoItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TodoItems {
    Table,
    Id,
    Title,
    Description,
    Completed,
    CreatedAt,
    UpdatedAt,
}
