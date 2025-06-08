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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_schema_sys_info_default() {
        let sys_info = SchemaSysInfo::default();
        assert_eq!(sys_info.timestamp, "");
        assert_eq!(sys_info.cpu_usage, 0.0);
        assert_eq!(sys_info.ram_usage, 0.0);
        assert_eq!(sys_info.total_ram, 0.0);
        assert_eq!(sys_info.free_ram, 0.0);
        assert_eq!(sys_info.used_swap, 0.0);
    }

    #[test]
    fn test_schema_disk_info_default() {
        let disk_info = SchemaDiskInfo::default();
        assert_eq!(disk_info.sysinfo_id, 0);
        assert_eq!(disk_info.filesystem, "");
        assert_eq!(disk_info.size, 0);
        assert_eq!(disk_info.used, 0);
        assert_eq!(disk_info.available, 0);
        assert_eq!(disk_info.used_percentage, 0);
        assert_eq!(disk_info.mounted_path, "");
    }

    #[test]
    fn test_schema_sys_info_creation() {
        let sys_info = SchemaSysInfo {
            timestamp: Utc::now().to_rfc3339(),
            cpu_usage: 25.5,
            ram_usage: 1024.0,
            total_ram: 8192.0,
            free_ram: 4096.0,
            used_swap: 512.0,
        };

        assert!(!sys_info.timestamp.is_empty());
        assert_eq!(sys_info.cpu_usage, 25.5);
        assert_eq!(sys_info.ram_usage, 1024.0);
        assert_eq!(sys_info.total_ram, 8192.0);
        assert_eq!(sys_info.free_ram, 4096.0);
        assert_eq!(sys_info.used_swap, 512.0);
    }

    #[test]
    fn test_schema_disk_info_creation() {
        let disk_info = SchemaDiskInfo {
            sysinfo_id: 1,
            filesystem: "ext4".to_string(),
            size: 1000,
            used: 500,
            available: 500,
            used_percentage: 50,
            mounted_path: "/".to_string(),
        };

        assert_eq!(disk_info.sysinfo_id, 1);
        assert_eq!(disk_info.filesystem, "ext4");
        assert_eq!(disk_info.size, 1000);
        assert_eq!(disk_info.used, 500);
        assert_eq!(disk_info.available, 500);
        assert_eq!(disk_info.used_percentage, 50);
        assert_eq!(disk_info.mounted_path, "/");
    }

    #[test]
    fn test_sys_info_serialization() {
        let sys_info = SysInfo {
            id: Some(1),
            timestamp: Utc::now().to_rfc3339(),
            cpu_usage: 25.5,
            ram_usage: 1024.0,
            total_ram: 8192.0,
            free_ram: 4096.0,
            used_swap: 512.0,
        };

        let serialized = serde_json::to_string(&sys_info).unwrap();
        assert!(serialized.contains("\"id\":1"));
        assert!(serialized.contains("\"cpu_usage\":25.5"));
        assert!(serialized.contains("\"ram_usage\":1024"));
    }

    #[test]
    fn test_disk_info_serialization() {
        let disk_info = DiskInfo {
            id: Some(1),
            sysinfo_id: 1,
            filesystem: "ext4".to_string(),
            size: 1000,
            used: 500,
            available: 500,
            used_percentage: 50,
            mounted_path: "/".to_string(),
        };

        let serialized = serde_json::to_string(&disk_info).unwrap();
        assert!(serialized.contains("\"id\":1"));
        assert!(serialized.contains("\"sysinfo_id\":1"));
        assert!(serialized.contains("\"filesystem\":\"ext4\""));
        assert!(serialized.contains("\"mounted_path\":\"/\""));
    }

    #[test]
    fn test_sys_info_debug_format() {
        let sys_info = SysInfo {
            id: Some(1),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            cpu_usage: 25.5,
            ram_usage: 1024.0,
            total_ram: 8192.0,
            free_ram: 4096.0,
            used_swap: 512.0,
        };

        let debug_str = format!("{:?}", sys_info);
        assert!(debug_str.contains("SysInfo"));
        assert!(debug_str.contains("25.5"));
        assert!(debug_str.contains("1024"));
    }

    #[test]
    fn test_edge_values() {
        // Test with extreme values
        let sys_info = SchemaSysInfo {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            cpu_usage: 100.0,
            ram_usage: 0.0,
            total_ram: f32::MAX,
            free_ram: f32::MAX,
            used_swap: 0.0,
        };

        assert_eq!(sys_info.cpu_usage, 100.0);
        assert_eq!(sys_info.ram_usage, 0.0);
        assert_eq!(sys_info.total_ram, f32::MAX);
        assert_eq!(sys_info.free_ram, f32::MAX);
        assert_eq!(sys_info.used_swap, 0.0);

        // Test disk info with edge values
        let disk_info = SchemaDiskInfo {
            sysinfo_id: i32::MAX,
            filesystem: "test".to_string(),
            size: i32::MAX,
            used: 0,
            available: i32::MAX,
            used_percentage: 100,
            mounted_path: "/test".to_string(),
        };

        assert_eq!(disk_info.sysinfo_id, i32::MAX);
        assert_eq!(disk_info.size, i32::MAX);
        assert_eq!(disk_info.used, 0);
        assert_eq!(disk_info.available, i32::MAX);
        assert_eq!(disk_info.used_percentage, 100);
    }

    #[test]
    fn test_deserialization() {
        let json_str = r#"{
            "timestamp": "2024-01-01T00:00:00Z",
            "cpu_usage": 25.5,
            "ram_usage": 1024.0,
            "total_ram": 8192.0,
            "free_ram": 4096.0,
            "used_swap": 512.0
        }"#;

        let sys_info: SchemaSysInfo = serde_json::from_str(json_str).unwrap();
        assert_eq!(sys_info.timestamp, "2024-01-01T00:00:00Z");
        assert_eq!(sys_info.cpu_usage, 25.5);
        assert_eq!(sys_info.ram_usage, 1024.0);
    }
}
