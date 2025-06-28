use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[allow(dead_code)]
pub type Bookmarks = Vec<Service>;

// For querying existing services from the database
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::services)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: Option<i32>,
    pub name: String,
    pub link: String,
    pub icon: Option<String>,
    pub user_id: i32,
}

// For inserting new services into the database
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::services)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct NewService {
    pub name: String,
    pub link: String,
    pub icon: Option<String>,
    pub user_id: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewServiceSchema {
    pub name: String,
    pub link: String,
    pub icon: Option<String>,
}

// For backwards compatibility if you still need BookmarkService
pub type BookmarkService = Service;
pub type ServicePayload = NewServiceSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServicePatchPayload {
    pub name: Option<String>,
    pub link: Option<String>,
    pub icon: Option<String>,
}
