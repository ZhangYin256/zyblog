use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Media::Table)
                    .col(
                        ColumnDef::new(Media::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Media::Filename)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Media::OriginalName)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Media::MimeType)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Media::SizeBytes)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Media::Path)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Media::UploadedBy).integer())
                    .col(
                        ColumnDef::new(Media::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_media_uploaded_by")
                            .from(Media::Table, Media::UploadedBy)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_media_uploaded_by")
                    .table(Media::Table)
                    .col(Media::UploadedBy)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Media::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Media {
    Table,
    Id,
    Filename,
    OriginalName,
    MimeType,
    SizeBytes,
    Path,
    UploadedBy,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
