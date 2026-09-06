use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Profiles {
    Table,
    Id,
    Name,
    Email,
    IdxProfilesEmail,
}

#[derive(Iden)]
pub enum Accounts {
    Table,
    Id,
    ProfileId,
    Name,
    Type,
    Balance,
    CurrencyCode,
    FkAccountsProfileId,
    IdxAccountsProfileId,
}

#[derive(Iden)]
pub enum PaymentMethods {
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
pub enum PaymentMethodConfig {
    Table,
    Id,
    PaymentMethodId,
    CreditLimit,
    AnnualEffectiveRate,
    BillingCycleDay,
    PaymentDueDateDay,
    FkPaymentMethodsConfigPaymentMethodId,
    IdxPaymentMethodConfigPaymentMethodId,
}

#[derive(Iden)]
pub enum DeviceProfileSessions {
    Table,
    Id,
    ProfileId,
    IsBiometricEnabled,
    BiometricSessionToken,
    UpdatedAt,
    IdxDeviceProfileSessionsProfileId,
    FkDeviceProfileSessionsProfilesId,
}

#[derive(Iden)]
pub enum Categories {
    Table,
    Id,
    ProfileId,
    Code,
    Name,
    ParentId,
    IdxCategoriesProfileId,
    IdxCategoriesParentId,
    IdxCategoriesUniqueProfileName,
    FkCategoriesProfileId,
    FkCategoriesParentId,
}

#[derive(Iden)]
pub enum Transactions {
    Table,
    Id,
    ProfileId,
    CategoryId,
    SourceAccountId,
    SourcePaymentMethodId,
    DestinationAccountId,
    Type,
    Status,
    Amount,
    FeeAmount,
    CurrencyCode,
    ExchangeRate,
    Notes,
    TransactionDate,
    CreatedAt,
    IdxTransactionsProfileId,
    IdxTransactionsSourceAccountId,
    IdxTransactionsDestinationAccountId,
    IdxTransactionsCategoryId,
    FkTransactionsProfileId,
    FkTransactionsCategoryId,
    FkTransactionsSourceAccountId,
    FkTransactionsSourcePaymentMethodId,
    FkTransactionsDestinationAccountId,
}

#[derive(Iden)]
pub enum CreditTransactionDetails {
    Table,
    Id,
    TransactionId,
    InstallmentsCount,
    AnnualEffectiveRate,
    IdxCreditTxDetailsTxId,
    FkCreditTxDetailsTxId,
}

#[derive(Iden)]
pub enum Loans {
    Table,
    Id,
    ProfileId,
    AccountId,
    Direction,
    ScheduleType,
    Name,
    PrincipalAmount,
    CurrencyCode,
    AnnualEffectiveRate,
    TermMonths,
    InstallmentsCount,
    InstallmentAmount,
    TotalAmountWithInterest,
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
pub enum LoanScheduleDaily {
    Table,
    Id,
    LoanId,
    EveryNDays,
    FkScheduleDailyLoanId,
    IdxScheduleDailyLoanId,
}

#[derive(Iden)]
pub enum LoanScheduleWeekly {
    Table,
    Id,
    LoanId,
    DayOfWeek,
    EveryNWeeks,
    FkScheduleWeeklyLoanId,
    IdxScheduleWeeklyLoanId,
}

#[derive(Iden)]
pub enum LoanSchedulePeriodic {
    Table,
    Id,
    LoanId,
    PeriodType,
    DayOfCycle,
    FkSchedulePeriodicLoanId,
    IdxSchedulePeriodicLoanId,
}

#[derive(Iden)]
pub enum LoanScheduleCustom {
    Table,
    Id,
    LoanId,
    IntervalValue,
    IntervalUnit,
    FkScheduleCustomLoanId,
    IdxScheduleCustomLoanId,
}

#[derive(Iden)]
pub enum LoanPayments {
    Table,
    Id,
    LoanId,
    TransactionId,
    PrincipalAmount,
    InterestAmount,
    LateFeeAmount,
    TotalAmount,
    PaymentDate,
    Notes,
    CreatedAt,
    FkLoanPaymentsLoanId,
    FkLoanPaymentsTransactionId,
    IdxLoanPaymentsLoanId,
    IdxLoanPaymentsTransactionId,
}

#[derive(Iden)]
pub enum ServiceFees {
    Table,
    Id,
    AccountId,
    PaymentMethodId,
    Type,
    Name,
    ThresholdAmount,
    IsActive,
    CreatedAt,
    IdxServiceFeesAccountId,
    IdxServiceFeesPaymentMethodId,
    FkServiceFeesAccountId,
    FkServiceFeesPaymentMethodId,
}

#[derive(Iden)]
pub enum ServiceFeeFixed {
    Table,
    Id,
    ServiceFeeId,
    Amount,
    PerUnitAmount,
    Currency,
    IdxServiceFeeFixedServiceFeeId,
    FkServiceFeeFixedServiceFeeId,
}

#[derive(Iden)]
pub enum ServiceFeePercentage {
    Table,
    Id,
    ServiceFeeId,
    Percentage,
    MinAmount,
    MaxAmount,
    IdxServiceFeePercentageServiceFeeId,
    FkServiceFeePercentageServiceFeeId,
}
