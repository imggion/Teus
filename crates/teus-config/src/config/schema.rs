use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use teus_schema::schema;

// You might also want structs for querying data later
#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = schema::config)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeusConfig {
    pub id: Option<i32>,
    pub first_visit: bool,
}
