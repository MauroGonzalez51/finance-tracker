use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260830_152950_create_accounts"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Accounts::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Accounts::ProfileId).uuid().not_null())
                    .col(ColumnDef::new(Accounts::Name).string_len(255).null())
                    .col(
                        ColumnDef::new(Accounts::Type)
                            .not_null()
                            .string_len(20)
                            .default("SAVING"),
                    )
                    .col(
                        ColumnDef::new(Accounts::Balance)
                            .text()
                            .not_null()
                            .default("0.00"),
                    )
                    .col(
                        ColumnDef::new(Accounts::CurrencyCode)
                            .string_len(3)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_accounts_profile_id")
                            .from(Accounts::Table, Accounts::ProfileId)
                            .to(Profiles::Table, Profiles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_accounts_profile_id")
                    .table(Accounts::Table)
                    .col(Accounts::ProfileId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Accounts {
    Table,
    Id,
    ProfileId,
    Name,
    Type,
    Balance,
    CurrencyCode,
}

#[derive(Iden)]
enum Profiles {
    Table,
    Id,
}
