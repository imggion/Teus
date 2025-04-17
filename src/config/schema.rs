use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// You might also want structs for querying data later
#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::config)]
pub struct TeusConfig {
    pub id: Option<i32>,
    pub first_visit: bool,
}
