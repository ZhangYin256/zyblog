use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PullRequestComments::Table)
                    .col(
                        ColumnDef::new(PullRequestComments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PullRequestComments::PullRequestId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PullRequestComments::Content).text().not_null())
                    .col(
                        ColumnDef::new(PullRequestComments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_pull_request_comments_pull_request_id")
                            .from(
                                PullRequestComments::Table,
                                PullRequestComments::PullRequestId,
                            )
                            .to(PullRequests::Table, PullRequests::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_pull_request_comments_pull_request_id")
                    .table(PullRequestComments::Table)
                    .col(PullRequestComments::PullRequestId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PullRequestComments::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PullRequestComments {
    Table,
    Id,
    PullRequestId,
    Content,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PullRequests {
    Table,
    Id,
}
