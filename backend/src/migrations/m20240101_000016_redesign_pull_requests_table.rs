use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Add fragments JSONB column
        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .add_column(ColumnDef::new(PullRequests::Fragments).json_binary().to_owned())
                    .to_owned(),
            )
            .await?;

        // Add user_id column with FK to users
        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .add_column(ColumnDef::new(PullRequests::UserId).integer().to_owned())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .add_foreign_key(
                        &TableForeignKey::new()
                            .name("fk_pull_requests_user_id")
                            .from_tbl(PullRequests::Table)
                            .from_col(PullRequests::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        // Add message TEXT column
        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .add_column(ColumnDef::new(PullRequests::Message).text().to_owned())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .drop_column(PullRequests::Fragments)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .drop_foreign_key(Alias::new("fk_pull_requests_user_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .drop_column(PullRequests::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(PullRequests::Table)
                    .drop_column(PullRequests::Message)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PullRequests {
    Table,
    Fragments,
    UserId,
    Message,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
