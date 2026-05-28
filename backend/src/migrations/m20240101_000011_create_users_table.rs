use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .col(
                        ColumnDef::new(Users::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::PasswordHash).string_len(255))
                    .col(
                        ColumnDef::new(Users::Name)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Users::AvatarUrl).string_len(500))
                    .col(
                        ColumnDef::new(Users::Role)
                            .string_len(20)
                            .not_null()
                            .default("visitor"),
                    )
                    .col(
                        ColumnDef::new(Users::GithubId)
                            .string_len(100)
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
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
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    PasswordHash,
    Name,
    AvatarUrl,
    Role,
    GithubId,
    CreatedAt,
    UpdatedAt,
}
