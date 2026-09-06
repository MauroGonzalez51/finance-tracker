use crate::{
    Accounts, IndexConfig, PaymentMethods, ServiceFeeFixed, ServiceFeePercentage, ServiceFees,
    create_indexes,
};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260906_153134_created_service_fee"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ServiceFees::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceFees::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ServiceFees::AccountId).uuid().null())
                    .col(ColumnDef::new(ServiceFees::PaymentMethodId).uuid().null())
                    .col(
                        ColumnDef::new(ServiceFees::Type)
                            .text()
                            .not_null()
                            .comment("FIXED, PERCENTAGE"),
                    )
                    .col(
                        ColumnDef::new(ServiceFees::Name)
                            .string()
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceFees::ThresholdAmount).text().null())
                    .col(
                        ColumnDef::new(ServiceFees::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ServiceFees::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(ServiceFees::FkServiceFeesAccountId.to_string())
                            .from(ServiceFees::Table, ServiceFees::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(ServiceFees::FkServiceFeesPaymentMethodId.to_string())
                            .from(ServiceFees::Table, ServiceFees::PaymentMethodId)
                            .to(PaymentMethods::Table, PaymentMethods::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .check(Check::unnamed(
                        Expr::col(ServiceFees::AccountId)
                            .is_not_null()
                            .or(Expr::col(ServiceFees::PaymentMethodId).is_not_null()),
                    ))
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            ServiceFees::Table,
            &[
                IndexConfig::new(ServiceFees::AccountId, ServiceFees::IdxServiceFeesAccountId),
                IndexConfig::new(
                    ServiceFees::PaymentMethodId,
                    ServiceFees::IdxServiceFeesPaymentMethodId,
                ),
            ],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(ServiceFeeFixed::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceFeeFixed::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ServiceFeeFixed::ServiceFeeId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ServiceFeeFixed::Amount).text().not_null())
                    .col(
                        ColumnDef::new(ServiceFeeFixed::PerUnitAmount)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceFeeFixed::Currency)
                            .string_len(3)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(ServiceFeeFixed::FkServiceFeeFixedServiceFeeId.to_string())
                            .from(ServiceFeeFixed::Table, ServiceFeeFixed::ServiceFeeId)
                            .to(ServiceFees::Table, ServiceFeeFixed::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            ServiceFeeFixed::Table,
            &[IndexConfig::new(
                ServiceFeeFixed::ServiceFeeId,
                ServiceFeeFixed::IdxServiceFeeFixedServiceFeeId,
            )
            .unique()],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(ServiceFeePercentage::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ServiceFeePercentage::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ServiceFeePercentage::ServiceFeeId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceFeePercentage::Percentage)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ServiceFeePercentage::MinAmount)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ServiceFeePercentage::MaxAmount)
                            .text()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(
                                ServiceFeePercentage::FkServiceFeePercentageServiceFeeId
                                    .to_string(),
                            )
                            .from(
                                ServiceFeePercentage::Table,
                                ServiceFeePercentage::ServiceFeeId,
                            )
                            .to(ServiceFees::Table, ServiceFees::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            ServiceFeePercentage::Table,
            &[IndexConfig::new(
                ServiceFeePercentage::ServiceFeeId,
                ServiceFeePercentage::IdxServiceFeePercentageServiceFeeId,
            )
            .unique()],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ServiceFeePercentage::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ServiceFeeFixed::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ServiceFees::Table).to_owned())
            .await?;

        Ok(())
    }
}
