use chrono::NaiveDateTime;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

use super::schema::{payment_method_configs, payment_methods};

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum PaymentMethodType {
    DebitCard,
    CreditCard,
    Transfer,
    CashPayment,
}

impl ToSql<Text, Sqlite> for PaymentMethodType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            PaymentMethodType::DebitCard => "DEBIT_CARD",
            PaymentMethodType::CreditCard => "CREDIT_CARD",
            PaymentMethodType::Transfer => "TRANSFER",
            PaymentMethodType::CashPayment => "CASH_PAYMENT",
        };
        out.set_value(value);
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for PaymentMethodType {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match value.as_str() {
            "DEBIT_CARD" => Ok(PaymentMethodType::DebitCard),
            "CREDIT_CARD" => Ok(PaymentMethodType::CreditCard),
            "TRANSFER" => Ok(PaymentMethodType::Transfer),
            "CASH_PAYMENT" => Ok(PaymentMethodType::CashPayment),
            other => Err(format!("unrecognized payment method type: {other}").into()),
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = payment_methods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PaymentMethod {
    pub id: String,
    pub account_id: String,
    pub type_: PaymentMethodType,
    pub card_number_last4: Option<String>,
    pub card_holder: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = payment_method_configs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PaymentMethodConfig {
    pub id: String,
    pub profile_id: String,
    pub payment_method_id: String,
    pub credit_limit: Option<String>,
    pub interest_rate: Option<String>,
    pub billing_cycle_day: Option<i32>,
}
