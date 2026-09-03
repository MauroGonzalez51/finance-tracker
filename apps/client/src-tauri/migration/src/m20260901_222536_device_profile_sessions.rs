use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260901_222536_device_profile_sessions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeviceProfileSessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeviceProfileSessions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DeviceProfileSessions::ProfileId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DeviceProfileSessions::IsBiometricEnabled)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(DeviceProfileSessions::BiometricSessionToken)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(DeviceProfileSessions::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(
                                DeviceProfileSessions::FkDeviceProfileSessionsProfilesId
                                    .to_string(),
                            )
                            .from(
                                DeviceProfileSessions::Table,
                                DeviceProfileSessions::ProfileId,
                            )
                            .to(Profiles::Table, Profiles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DeviceProfileSessions::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum DeviceProfileSessions {
    Table,
    Id,
    ProfileId,
    IsBiometricEnabled,
    BiometricSessionToken,
    UpdatedAt,
    FkDeviceProfileSessionsProfilesId,
}

#[derive(Iden)]
enum Profiles {
    Table,
    Id,
}
