use crate::{
    Accounts, Categories, CreditTransactionDetails, IndexConfig, PaymentMethods, Profiles,
    Transactions, create_indexes,
};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260902_130334_create_transactions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Transactions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Transactions::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Transactions::ProfileId).uuid().not_null())
                    .col(ColumnDef::new(Transactions::CategoryId).uuid().null())
                    .col(ColumnDef::new(Transactions::SourceAccountId).uuid().null())
                    .col(
                        ColumnDef::new(Transactions::SourcePaymentMethodId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Transactions::DestinationAccountId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Transactions::Type)
                            .string()
                            .not_null()
                            .comment("EXPENSE, INCOME, TRANSFER, LOAN_PAYMENT"),
                    )
                    .col(
                        ColumnDef::new(Transactions::Status)
                            .string()
                            .not_null()
                            .default("COMPLETED")
                            .comment("PENDING, COMPLETED, VOIDED"),
                    )
                    .col(ColumnDef::new(Transactions::Amount).text().not_null())
                    .col(
                        ColumnDef::new(Transactions::FeeAmount)
                            .text()
                            .not_null()
                            .default("0.00"),
                    )
                    .col(
                        ColumnDef::new(Transactions::CurrencyCode)
                            .string_len(3)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Transactions::ExchangeRate)
                            .text()
                            .not_null()
                            .default("1.00"),
                    )
                    .col(ColumnDef::new(Transactions::Notes).text().null())
                    .col(
                        ColumnDef::new(Transactions::TransactionDate)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(
                        ColumnDef::new(Transactions::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Transactions::FkTransactionsProfileId.to_string())
                            .from(Transactions::Table, Transactions::ProfileId)
                            .to(Profiles::Table, Profiles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Transactions::FkTransactionsCategoryId.to_string())
                            .from(Transactions::Table, Transactions::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Transactions::FkTransactionsSourceAccountId.to_string())
                            .from(Transactions::Table, Transactions::SourceAccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Transactions::FkTransactionsSourcePaymentMethodId.to_string())
                            .from(Transactions::Table, Transactions::SourcePaymentMethodId)
                            .to(PaymentMethods::Table, PaymentMethods::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Transactions::FkTransactionsDestinationAccountId.to_string())
                            .from(Transactions::Table, Transactions::DestinationAccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            Transactions::Table,
            &[
                IndexConfig::new(
                    Transactions::ProfileId,
                    Transactions::IdxTransactionsProfileId,
                ),
                IndexConfig::new(
                    Transactions::SourceAccountId,
                    Transactions::IdxTransactionsSourceAccountId,
                ),
                IndexConfig::new(
                    Transactions::DestinationAccountId,
                    Transactions::IdxTransactionsDestinationAccountId,
                ),
                IndexConfig::new(
                    Transactions::CategoryId,
                    Transactions::IdxTransactionsCategoryId,
                ),
            ],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(CreditTransactionDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CreditTransactionDetails::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CreditTransactionDetails::TransactionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CreditTransactionDetails::InstallmentsCount)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(
                        ColumnDef::new(CreditTransactionDetails::AnnualEffectiveRate)
                            .text()
                            .not_null()
                            .default("0.00"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(CreditTransactionDetails::FkCreditTxDetailsTxId.to_string())
                            .from(
                                CreditTransactionDetails::Table,
                                CreditTransactionDetails::TransactionId,
                            )
                            .to(Transactions::Table, Transactions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            CreditTransactionDetails::Table,
            &[IndexConfig::new(
                CreditTransactionDetails::TransactionId,
                CreditTransactionDetails::IdxCreditTxDetailsTxId,
            )
            .unique()],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(CreditTransactionDetails::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await?;

        Ok(())
    }
}
