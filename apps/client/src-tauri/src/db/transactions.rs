use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

use super::schema::{transaction_configs, transactions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum TransactionType {
    InitialBalance,
    Deposit,
    Withdraw,
}

impl ToSql<Text, Sqlite> for TransactionType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            TransactionType::InitialBalance => "INITIAL_BALANCE",
            TransactionType::Deposit => "DEPOSIT",
            TransactionType::Withdraw => "WITHDRAW",
        };
        out.set_value(value);
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for TransactionType {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match value.as_str() {
            "INITIAL_BALANCE" => Ok(TransactionType::InitialBalance),
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "WITHDRAW" => Ok(TransactionType::Withdraw),
            other => Err(format!("unrecognized transaction type: {other}").into()),
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Transaction {
    pub id: String,
    pub profile_id: String,
    pub account_id: Option<String>,
    pub payment_method_id: Option<String>,
    pub category_id: Option<String>,
    pub type_: TransactionType,
    pub amount: String,
    pub notes: Option<String>,
    pub date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = transaction_configs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TransactionConfig {
    pub id: String,
    pub profile_id: String,
    pub transaction_id: String,
    pub payment_method_id: Option<String>,
    pub installments: i32,
    pub credit_limit: Option<String>,
    pub interest_rate: Option<String>,
    pub billing_cycle_day: Option<i32>,
}
