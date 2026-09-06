use crate::{Accounts, IndexConfig, PaymentMethodConfig, PaymentMethods, create_indexes};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260901_211119_payment_methods"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentMethods::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentMethods::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PaymentMethods::AccountId).uuid().not_null())
                    .col(
                        ColumnDef::new(PaymentMethods::Type)
                            .text()
                            .not_null()
                            .default("CASH_PAYMENT")
                            .comment("DEBIT_CARD, CREDIT_CARD, TRANSFER, CASH_PAYMENT"),
                    )
                    .col(
                        ColumnDef::new(PaymentMethods::CardNumberLast4)
                            .string_len(4)
                            .null(),
                    )
                    .col(ColumnDef::new(PaymentMethods::CardHolder).string().null())
                    .col(
                        ColumnDef::new(PaymentMethods::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(PaymentMethods::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(PaymentMethods::FkPaymentMethodsAccountId.to_string())
                            .from(PaymentMethods::Table, PaymentMethods::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            PaymentMethods::Table,
            &[IndexConfig::new(
                PaymentMethods::AccountId,
                PaymentMethods::IdxPaymentMethodsAccountId,
            )],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(PaymentMethodConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentMethodConfig::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethodConfig::PaymentMethodId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethodConfig::CreditLimit)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethodConfig::AnnualEffectiveRate)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethodConfig::BillingCycleDay)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethodConfig::PaymentDueDateDay)
                            .integer()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(
                                PaymentMethodConfig::FkPaymentMethodsConfigPaymentMethodId
                                    .to_string(),
                            )
                            .from(
                                PaymentMethodConfig::Table,
                                PaymentMethodConfig::PaymentMethodId,
                            )
                            .to(PaymentMethods::Table, PaymentMethods::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            PaymentMethodConfig::Table,
            &[IndexConfig::new(
                PaymentMethodConfig::PaymentMethodId,
                PaymentMethodConfig::IdxPaymentMethodConfigPaymentMethodId,
            )
            .unique()],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PaymentMethodConfig::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PaymentMethods::Table).to_owned())
            .await?;

        Ok(())
    }
}
