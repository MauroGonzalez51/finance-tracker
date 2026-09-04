use crate::create_indexes;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260829_190325_init"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profiles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Profiles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Profiles::Name).text().not_null())
                    .col(ColumnDef::new(Profiles::Email).text().not_null())
                    .to_owned(),
            )
            .await?;

        create_indexes!(
            manager,
            Profiles::Table,
            [Profiles::Email => Profiles::IdxProfilesEmail]
        );

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profiles::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Profiles {
    Table,
    Id,
    Name,
    Email,
    IdxProfilesEmail,
}
