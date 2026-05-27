use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PullRequests::Table)
                    .col(
                        ColumnDef::new(PullRequests::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PullRequests::PostId).integer().not_null())
                    .col(ColumnDef::new(PullRequests::UserEmail).string().not_null())
                    .col(ColumnDef::new(PullRequests::Content).text().not_null())
                    .col(
                        ColumnDef::new(PullRequests::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(PullRequests::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_pull_requests_post_id")
                            .from(PullRequests::Table, PullRequests::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_pull_requests_post_id")
                    .table(PullRequests::Table)
                    .col(PullRequests::PostId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_pull_requests_status")
                    .table(PullRequests::Table)
                    .col(PullRequests::Status)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PullRequests::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PullRequests {
    Table,
    Id,
    PostId,
    UserEmail,
    Content,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}
