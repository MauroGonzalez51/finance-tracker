use diesel::prelude::*;

use super::schema::categories;

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: String,
    pub profile_id: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
}
