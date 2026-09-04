use crate::create_indexes;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260904_212146_create_loan"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Loans::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Loans::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Loans::ProfileId).uuid().not_null())
                    .col(ColumnDef::new(Loans::AccountId).uuid().not_null())
                    .col(
                        ColumnDef::new(Loans::Direction)
                            .text()
                            .not_null()
                            .default("BORROWED"),
                    )
                    .col(ColumnDef::new(Loans::Name).string_len(255).null())
                    .col(ColumnDef::new(Loans::PrincipalAmount).text().not_null())
                    .col(ColumnDef::new(Loans::CurrencyCode).string_len(3).not_null())
                    .col(ColumnDef::new(Loans::InterestRate).text().not_null())
                    .col(
                        ColumnDef::new(Loans::InterestType)
                            .text()
                            .not_null()
                            .default("SIMPLE"),
                    )
                    .col(
                        ColumnDef::new(Loans::NumberOfInstallments)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Loans::InstallmentAmount).text().not_null())
                    .col(
                        ColumnDef::new(Loans::TotalAmountWithInterest)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Loans::TermMonths).integer().not_null())
                    .col(
                        ColumnDef::new(Loans::StartDate)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(
                        ColumnDef::new(Loans::FirstPaymentDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Loans::DueDate).date_time().not_null())
                    .col(
                        ColumnDef::new(Loans::Status)
                            .text()
                            .not_null()
                            .default("ACTIVE"),
                    )
                    .col(ColumnDef::new(Loans::Notes).text().null())
                    .col(
                        ColumnDef::new(Loans::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(
                        ColumnDef::new(Loans::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Loans::FkLoansProfileId.to_string())
                            .from(Loans::Table, Loans::ProfileId)
                            .to(Profiles::Table, Profiles::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(Loans::FkLoansAccountId.to_string())
                            .from(Loans::Table, Loans::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes!(
            manager,
            Loans::Table,
            [
                Loans::ProfileId => Loans::IdxLoansProfileId,
                Loans::AccountId => Loans::IdxLoansAccountId
            ]
        );

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Loans::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Loans {
    Table,
    Id,
    ProfileId,
    AccountId,
    Direction,
    Name,
    PrincipalAmount,
    CurrencyCode,
    InterestRate,
    InterestType,
    NumberOfInstallments,
    InstallmentAmount,
    TotalAmountWithInterest,
    TermMonths,
    StartDate,
    FirstPaymentDate,
    DueDate,
    Status,
    Notes,
    CreatedAt,
    UpdatedAt,
    FkLoansProfileId,
    FkLoansAccountId,
    IdxLoansProfileId,
    IdxLoansAccountId,
}

#[derive(Iden)]
enum Accounts {
    Table,
    Id,
}

#[derive(Iden)]
enum Profiles {
    Table,
    Id,
}
