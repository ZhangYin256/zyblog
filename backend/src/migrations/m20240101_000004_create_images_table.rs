use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Images::Table)
                    .col(
                        ColumnDef::new(Images::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Images::Filename).string().not_null())
                    .col(ColumnDef::new(Images::OriginalName).string().not_null())
                    .col(ColumnDef::new(Images::MimeType).string().not_null())
                    .col(ColumnDef::new(Images::Size).integer().not_null())
                    .col(ColumnDef::new(Images::Path).string().not_null())
                    .col(
                        ColumnDef::new(Images::CreatedAt)
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
            .drop_table(Table::drop().table(Images::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Images {
    Table,
    Id,
    Filename,
    OriginalName,
    MimeType,
    Size,
    Path,
    CreatedAt,
}
