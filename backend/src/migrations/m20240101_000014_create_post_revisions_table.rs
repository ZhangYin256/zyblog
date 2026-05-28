use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostRevisions::Table)
                    .col(
                        ColumnDef::new(PostRevisions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PostRevisions::PostId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PostRevisions::Title).string_len(500))
                    .col(ColumnDef::new(PostRevisions::Content).text())
                    .col(ColumnDef::new(PostRevisions::Excerpt).text())
                    .col(ColumnDef::new(PostRevisions::CoverImage).string_len(500))
                    .col(
                        ColumnDef::new(PostRevisions::Version)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PostRevisions::CreatedBy).integer())
                    .col(
                        ColumnDef::new(PostRevisions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_revisions_post_id")
                            .from(PostRevisions::Table, PostRevisions::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_revisions_created_by")
                            .from(PostRevisions::Table, PostRevisions::CreatedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_post_revisions_post_id")
                    .table(PostRevisions::Table)
                    .col(PostRevisions::PostId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_post_revisions_version")
                    .table(PostRevisions::Table)
                    .col(PostRevisions::PostId)
                    .col(PostRevisions::Version)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostRevisions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostRevisions {
    Table,
    Id,
    PostId,
    Title,
    Content,
    Excerpt,
    CoverImage,
    Version,
    CreatedBy,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
