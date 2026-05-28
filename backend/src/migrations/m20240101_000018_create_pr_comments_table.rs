use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PrComments::Table)
                    .col(
                        ColumnDef::new(PrComments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PrComments::PullRequestId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PrComments::FragmentIndex).integer())
                    .col(ColumnDef::new(PrComments::Line).integer())
                    .col(ColumnDef::new(PrComments::Content).text().not_null())
                    .col(ColumnDef::new(PrComments::UserId).integer())
                    .col(
                        ColumnDef::new(PrComments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_pr_comments_pull_request_id")
                            .from(PrComments::Table, PrComments::PullRequestId)
                            .to(PullRequests::Table, PullRequests::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        &mut ForeignKey::create()
                            .name("fk_pr_comments_user_id")
                            .from(PrComments::Table, PrComments::UserId)
                            .to(Users::Table, Users::Id)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_pr_comments_pull_request_id")
                    .table(PrComments::Table)
                    .col(PrComments::PullRequestId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PrComments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PrComments {
    Table,
    Id,
    PullRequestId,
    FragmentIndex,
    Line,
    Content,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum PullRequests {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
