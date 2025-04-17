// src/monitor/schema.rs
use crate::schema::{diskinfo, sysinfo};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = sysinfo)]
pub struct SchemaSysInfo {
    pub timestamp: String,
    pub cpu_usage: f32, // Changed to f32 to match schema
    pub ram_usage: f32, // Changed to f32 to match schema
    pub total_ram: f32, // Changed to f32 to match schema
    pub free_ram: f32,  // Changed to f32 to match schema
    pub used_swap: f32, // Changed to f32 to match schema
    // pub user_id: i32,   // Assuming user_id is i32
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = diskinfo)]
pub struct SchemaDiskInfo {
    pub sysinfo_id: i32,
    pub filesystem: String,
    pub size: i32,      // Changed to i32 to match schema (Integer maps to i32 often)
    pub used: i32,      // Changed to i32
    pub available: i32, // Changed to i32
    pub used_percentage: i32, // Changed to i32
    pub mounted_path: String,
}

// You might also want structs for querying data later
#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = sysinfo)]
pub struct SysInfo {
    #[diesel(column_name = id)] // Explicitly map id if needed, depends on schema generation
    pub id: Option<i32>,
    pub timestamp: String,
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub total_ram: f32,
    pub free_ram: f32,
    pub used_swap: f32,
    // pub user_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = diskinfo)]
pub struct DiskInfo {
    #[diesel(column_name = id)] // Explicitly map id
    pub id: Option<i32>,
    pub sysinfo_id: i32,
    pub filesystem: String,
    pub size: i32,      // Changed to i32 to match schema (Integer maps to i32 often)
    pub used: i32,      // Changed to i32
    pub available: i32, // Changed to i32
    pub used_percentage: i32,
    pub mounted_path: String,
}

impl Default for SchemaSysInfo {
    fn default() -> Self {
        Self {
            timestamp: "".to_string(),
            cpu_usage: 0.0,
            ram_usage: 0.0,
            total_ram: 0.0,
            free_ram: 0.0,
            used_swap: 0.0,
            // user_id: 0,
        }
    }
}

impl Default for SchemaDiskInfo {
    fn default() -> Self {
        Self {
            sysinfo_id: 0,
            filesystem: "".to_string(),
            size: 0,
            used: 0,
            available: 0,
            used_percentage: 0,
            mounted_path: "".to_string(),
        }
    }
}
