use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Comments::Table)
                    .add_column(ColumnDef::new(Comments::ReferencedContent).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Comments::Table)
                    .drop_column(Comments::ReferencedContent)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Comments {
    Table,
    ReferencedContent,
}
