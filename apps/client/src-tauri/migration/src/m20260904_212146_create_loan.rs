use crate::{
    Accounts, IndexConfig, LoanPayments, LoanScheduleCustom, LoanScheduleDaily,
    LoanSchedulePeriodic, LoanScheduleWeekly, Loans, Profiles, Transactions, create_indexes,
};
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
                    .col(ColumnDef::new(Loans::AccountId).uuid().null())
                    .col(
                        ColumnDef::new(Loans::Direction)
                            .text()
                            .not_null()
                            .default("BORROWED")
                            .comment("BORROWED, LENT"),
                    )
                    .col(
                        ColumnDef::new(Loans::ScheduleType)
                            .text()
                            .not_null()
                            .default("PERIODIC")
                            .comment("DAILY, WEEKLY, PERIODIC, CUSTOM, AT_MATURITY"),
                    )
                    .col(ColumnDef::new(Loans::Name).string_len(255).null())
                    .col(ColumnDef::new(Loans::PrincipalAmount).text().not_null())
                    .col(ColumnDef::new(Loans::CurrencyCode).string_len(3).not_null())
                    .col(
                        ColumnDef::new(Loans::AnnualEffectiveRate)
                            .text()
                            .not_null()
                            .default("0.00"),
                    )
                    .col(ColumnDef::new(Loans::TermMonths).integer().not_null())
                    .col(
                        ColumnDef::new(Loans::InstallmentsCount)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Loans::InstallmentAmount).text().not_null())
                    .col(
                        ColumnDef::new(Loans::TotalAmountWithInterest)
                            .text()
                            .not_null(),
                    )
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
                            .default("ACTIVE")
                            .comment("ACTIVE, PAID, DEFAULTED, CANCELLED"),
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

        create_indexes(
            manager,
            Loans::Table,
            &[
                IndexConfig::new(Loans::ProfileId, Loans::IdxLoansProfileId),
                IndexConfig::new(Loans::AccountId, Loans::IdxLoansAccountId),
            ],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(LoanScheduleDaily::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoanScheduleDaily::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LoanScheduleDaily::LoanId).uuid().not_null())
                    .col(
                        ColumnDef::new(LoanScheduleDaily::EveryNDays)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(LoanScheduleDaily::FkScheduleDailyLoanId.to_string())
                            .from(LoanScheduleDaily::Table, LoanScheduleDaily::LoanId)
                            .to(Loans::Table, Loans::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            LoanScheduleDaily::Table,
            &[IndexConfig::new(
                LoanScheduleDaily::LoanId,
                LoanScheduleDaily::IdxScheduleDailyLoanId,
            )
            .unique()],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(LoanScheduleWeekly::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoanScheduleWeekly::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LoanScheduleWeekly::LoanId).uuid().not_null())
                    .col(
                        ColumnDef::new(LoanScheduleWeekly::DayOfWeek)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoanScheduleWeekly::EveryNWeeks)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(LoanScheduleWeekly::FkScheduleWeeklyLoanId.to_string())
                            .from(LoanScheduleWeekly::Table, LoanScheduleWeekly::LoanId)
                            .to(Loans::Table, Loans::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            LoanScheduleWeekly::Table,
            &[IndexConfig::new(
                LoanScheduleWeekly::LoanId,
                LoanScheduleWeekly::IdxScheduleWeeklyLoanId,
            )
            .unique()],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(LoanSchedulePeriodic::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoanSchedulePeriodic::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(LoanSchedulePeriodic::LoanId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoanSchedulePeriodic::PeriodType)
                            .text()
                            .not_null()
                            .comment("MONTHLY, BIMONTHLY, QUARTERLY, SEMESTRLY, ANNUAL"),
                    )
                    .col(
                        ColumnDef::new(LoanSchedulePeriodic::DayOfCycle)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(LoanSchedulePeriodic::FkSchedulePeriodicLoanId.to_string())
                            .from(LoanSchedulePeriodic::Table, LoanSchedulePeriodic::LoanId)
                            .to(Loans::Table, Loans::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            LoanSchedulePeriodic::Table,
            &[IndexConfig::new(
                LoanSchedulePeriodic::LoanId,
                LoanSchedulePeriodic::IdxSchedulePeriodicLoanId,
            )
            .unique()],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(LoanScheduleCustom::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoanScheduleCustom::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LoanScheduleCustom::LoanId).uuid().not_null())
                    .col(
                        ColumnDef::new(LoanScheduleCustom::IntervalValue)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoanScheduleCustom::IntervalUnit)
                            .text()
                            .not_null()
                            .comment("DAYS, WEEKS, MONTHS"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(LoanScheduleCustom::FkScheduleCustomLoanId.to_string())
                            .from(LoanScheduleCustom::Table, LoanScheduleCustom::LoanId)
                            .to(Loans::Table, Loans::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(
            manager,
            LoanScheduleCustom::Table,
            &[IndexConfig::new(
                LoanScheduleCustom::LoanId,
                LoanScheduleCustom::IdxScheduleCustomLoanId,
            )
            .unique()],
        )
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(LoanPayments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(LoanPayments::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(LoanPayments::LoanId).uuid().not_null())
                    .col(
                        ColumnDef::new(LoanPayments::TransactionId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoanPayments::PrincipalAmount)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoanPayments::InterestAmount)
                            .string()
                            .not_null()
                            .default("0.00"),
                    )
                    .col(
                        ColumnDef::new(LoanPayments::LateFeeAmount)
                            .string()
                            .not_null()
                            .default("0.00"),
                    )
                    .col(
                        ColumnDef::new(LoanPayments::TotalAmount)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(LoanPayments::PaymentDate)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .col(ColumnDef::new(LoanPayments::Notes).string().null())
                    .col(
                        ColumnDef::new(LoanPayments::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::cust("CURRENT_TIMESTAMP")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(LoanPayments::FkLoanPaymentsLoanId.to_string())
                            .from(LoanPayments::Table, LoanPayments::LoanId)
                            .to(Loans::Table, Loans::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(LoanPayments::FkLoanPaymentsTransactionId.to_string())
                            .from(LoanPayments::Table, LoanPayments::TransactionId)
                            .to(Transactions::Table, Transactions::Id)
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
            .drop_table(Table::drop().table(LoanPayments::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(LoanScheduleCustom::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(LoanSchedulePeriodic::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(LoanScheduleWeekly::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(LoanScheduleDaily::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Loans::Table).to_owned())
            .await?;

        Ok(())
    }
}
