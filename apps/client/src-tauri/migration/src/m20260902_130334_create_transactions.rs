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

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
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
