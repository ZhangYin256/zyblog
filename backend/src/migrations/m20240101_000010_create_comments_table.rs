use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comments::Table)
                    .col(
                        ColumnDef::new(Comments::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Comments::PostId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Comments::AuthorName)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Comments::AuthorEmail).string())
                    .col(ColumnDef::new(Comments::Content).text().not_null())
                    .col(
                        ColumnDef::new(Comments::Approved)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Comments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Comments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_comments_post_id")
                            .from(Comments::Table, Comments::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_comments_post_id")
                    .table(Comments::Table)
                    .col(Comments::PostId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comments::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    Id,
    PostId,
    AuthorName,
    AuthorEmail,
    Content,
    Approved,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}
