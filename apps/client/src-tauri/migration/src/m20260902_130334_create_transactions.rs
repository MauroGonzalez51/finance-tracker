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
                    .col(ColumnDef::new(Transactions::AccountId).uuid().null())
                    .col(ColumnDef::new(Transactions::PaymentMethodId).uuid().null())
                    .col(ColumnDef::new(Transactions::CategoryId).uuid().null())
                    .col(ColumnDef::new(Transactions::Type).text().not_null())
                    .col(ColumnDef::new(Transactions::Amount).text().not_null())
                    .col(ColumnDef::new(Transactions::Notes).text().null())
                    .col(
                        ColumnDef::new(Transactions::Date)
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
                            .name(Transactions::FkTransactionsAccountId.to_string())
                            .from(Transactions::Table, Transactions::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Transactions::FkTransactionsPaymentMethodId.to_string())
                            .from(Transactions::Table, Transactions::PaymentMethodId)
                            .to(PaymentMethods::Table, PaymentMethods::Id)
                            .on_delete(ForeignKeyAction::SetNull)
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
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Transactions::IdxTransactionsAccountId.to_string())
                    .if_not_exists()
                    .table(Transactions::Table)
                    .col(Transactions::AccountId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Transactions::IdxTransactionsPaymentMethodId.to_string())
                    .if_not_exists()
                    .table(Transactions::Table)
                    .col(Transactions::PaymentMethodId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Transactions::IdxTransactionsCategoryId.to_string())
                    .if_not_exists()
                    .table(Transactions::Table)
                    .col(Transactions::CategoryId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransactionConfig::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TransactionConfig::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TransactionConfig::TransactionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TransactionConfig::Installments)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(ColumnDef::new(TransactionConfig::CreditLimit).text().null())
                    .col(
                        ColumnDef::new(TransactionConfig::InterestRate)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TransactionConfig::BillingCycleDay)
                            .integer()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(TransactionConfig::FkTransactionConfigTransactionId.to_string())
                            .from(TransactionConfig::Table, TransactionConfig::TransactionId)
                            .to(Transactions::Table, Transactions::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(TransactionConfig::IdxTransactionConfigTransactionId.to_string())
                    .if_not_exists()
                    .table(TransactionConfig::Table)
                    .col(TransactionConfig::TransactionId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Transactions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TransactionConfig::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Transactions {
    Table,
    Id,
    AccountId,
    PaymentMethodId,
    CategoryId,
    Type,
    Amount,
    Notes,
    Date,
    CreatedAt,
    IdxTransactionsAccountId,
    IdxTransactionsPaymentMethodId,
    IdxTransactionsCategoryId,
    FkTransactionsAccountId,
    FkTransactionsPaymentMethodId,
    FkTransactionsCategoryId,
}

#[derive(Iden)]
enum TransactionConfig {
    Table,
    Id,
    TransactionId,
    Installments,
    CreditLimit,
    InterestRate,
    BillingCycleDay,
    IdxTransactionConfigTransactionId,
    FkTransactionConfigTransactionId,
}

#[derive(Iden)]
enum Accounts {
    Table,
    Id,
}

#[derive(Iden)]
enum PaymentMethods {
    Table,
    Id,
}

#[derive(Iden)]
enum Categories {
    Table,
    Id,
}
