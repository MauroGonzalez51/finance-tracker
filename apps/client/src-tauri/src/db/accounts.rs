use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

use super::schema::accounts;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum AccountType {
    Checking,
    Saving,
    Cash,
    DigitalWallet,
}

impl ToSql<Text, Sqlite> for AccountType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let value = match self {
            AccountType::Checking => "CHECKING",
            AccountType::Saving => "SAVING",
            AccountType::Cash => "CASH",
            AccountType::DigitalWallet => "DIGITAL_WALLET",
        };
        out.set_value(value);
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for AccountType {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let value = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        match value.as_str() {
            "CHECKING" => Ok(AccountType::Checking),
            "SAVING" => Ok(AccountType::Saving),
            "CASH" => Ok(AccountType::Cash),
            "DIGITAL_WALLET" => Ok(AccountType::DigitalWallet),
            other => Err(format!("unrecognized account type: {other}").into()),
        }
    }
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Account {
    pub id: String,
    pub profile_id: String,
    pub name: Option<String>,
    pub type_: AccountType,
    pub balance: String,
    pub currency_code: String,
}
