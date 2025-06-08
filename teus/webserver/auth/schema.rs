use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable, Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
    pub salt: String,
}