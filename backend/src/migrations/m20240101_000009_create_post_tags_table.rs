use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PostTags::Table)
                    .col(ColumnDef::new(PostTags::PostId).integer().not_null())
                    .col(ColumnDef::new(PostTags::TagId).integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(PostTags::PostId)
                            .col(PostTags::TagId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tags_post_id")
                            .from(PostTags::Table, PostTags::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tags_tag_id")
                            .from(PostTags::Table, PostTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_post_tags_tag_id")
                    .table(PostTags::Table)
                    .col(PostTags::TagId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PostTags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PostTags {
    Table,
    PostId,
    TagId,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Tags {
    Table,
    Id,
}
