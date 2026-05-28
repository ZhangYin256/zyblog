use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .add_column(ColumnDef::new(Posts::AuthorId).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_posts_author_id")
                            .from_tbl(Posts::Table)
                            .from_col(Posts::AuthorId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .drop_foreign_key(Alias::new("fk_posts_author_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Posts::Table)
                    .drop_column(Posts::AuthorId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    AuthorId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
