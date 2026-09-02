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
                            .default("CASH_PAYMENT"),
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

        manager
            .create_index(
                Index::create()
                    .name(PaymentMethods::IdxPaymentMethodsAccountId.to_string())
                    .if_not_exists()
                    .table(PaymentMethods::Table)
                    .col(PaymentMethods::AccountId)
                    .to_owned(),
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
                        ColumnDef::new(PaymentMethodConfig::InterestRate)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(PaymentMethodConfig::BillingCycleDay)
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

        manager
            .create_index(
                Index::create()
                    .name(PaymentMethodConfig::IdxPaymentMethodConfigPaymentMethodId.to_string())
                    .if_not_exists()
                    .table(PaymentMethodConfig::Table)
                    .col(PaymentMethodConfig::PaymentMethodId)
                    .to_owned(),
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

#[derive(Iden)]
enum PaymentMethods {
    Table,
    Id,
    AccountId,
    Type,
    CardNumberLast4,
    CardHolder,
    IsActive,
    CreatedAt,
    FkPaymentMethodsAccountId,
    IdxPaymentMethodsAccountId,
}

#[derive(Iden)]
enum PaymentMethodConfig {
    Table,
    Id,
    PaymentMethodId,
    CreditLimit,
    InterestRate,
    BillingCycleDay,
    FkPaymentMethodsConfigPaymentMethodId,
    IdxPaymentMethodConfigPaymentMethodId,
}

#[derive(Iden)]
enum Accounts {
    Table,
    Id,
}
