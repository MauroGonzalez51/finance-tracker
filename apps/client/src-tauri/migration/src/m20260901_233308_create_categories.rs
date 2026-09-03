use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260901_233308_create_categories"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Categories::ProfileId).uuid().null())
                    .col(ColumnDef::new(Categories::Code).string_len(255).null())
                    .col(ColumnDef::new(Categories::Name).string_len(255).null())
                    .col(ColumnDef::new(Categories::ParentId).uuid().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(Categories::FkCategoriesProfileId.to_string())
                            .from(Categories::Table, Categories::ProfileId)
                            .to(Profiles::Table, Profiles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Categories::FkCategoriesParentId.to_string())
                            .from(Categories::Table, Categories::ParentId)
                            .to(Categories::Table, Categories::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Categories::IdxCategoriesProfileId.to_string())
                    .if_not_exists()
                    .table(Categories::Table)
                    .col(Categories::ProfileId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Categories::IdxCategoriesParentId.to_string())
                    .if_not_exists()
                    .table(Categories::Table)
                    .col(Categories::ParentId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
    ProfileId,
    Code,
    Name,
    ParentId,
    IdxCategoriesProfileId,
    IdxCategoriesParentId,
    FkCategoriesProfileId,
    FkCategoriesParentId,
}

#[derive(Iden)]
enum Profiles {
    Table,
    Id,
}
