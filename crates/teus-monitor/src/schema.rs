//! Database schema structures for system monitoring data.
//!
//! This module defines the data structures used to store and retrieve
//! system monitoring information in the SQLite database. It includes
//! both insertable structures for writing new data and queryable
//! structures for reading existing data.

use teus_schema::schema::{diskinfo, sysinfo};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Structure for inserting system information records into the database.
///
/// This structure represents a snapshot of system resource usage at a
/// specific point in time. It's designed to be inserted into the `sysinfo`
/// table and serves as the primary record for system monitoring data.
///
/// # Database Schema
///
/// Maps to the `sysinfo` table with the following constraints:
/// - `timestamp` should be in RFC3339 format for consistency
/// - All usage values are stored as floating-point numbers for precision
/// - Memory values are typically stored in bytes or megabytes
///
/// # Examples
///
/// ```rust
/// use teus::monitor::schema::SchemaSysInfo;
/// use chrono::Utc;
///
/// let sys_info = SchemaSysInfo {
///     timestamp: Utc::now().to_rfc3339(),
///     cpu_usage: 25.5,
///     ram_usage: 4096.0,
///     total_ram: 16384.0,
///     free_ram: 8192.0,
///     used_swap: 512.0,
/// };
/// ```
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = sysinfo)]
pub struct SchemaSysInfo {
    /// Timestamp when this system information was collected.
    ///
    /// Should be in RFC3339 format (e.g., "2024-01-01T12:00:00Z")
    /// for consistent parsing and sorting.
    pub timestamp: String,

    /// CPU usage percentage at the time of collection.
    ///
    /// Range: 0.0 to 100.0, where 100.0 represents full CPU utilization.
    pub cpu_usage: f32,

    /// Amount of RAM currently in use, in megabytes.
    ///
    /// This represents the memory actively being used by processes,
    /// excluding cached and buffered memory.
    pub ram_usage: f32,

    /// Total amount of RAM available in the system, in megabytes.
    ///
    /// This is the physical memory capacity and should remain
    /// relatively constant unless hardware changes occur.
    pub total_ram: f32,

    /// Amount of RAM currently free and available, in megabytes.
    ///
    /// This represents memory that is immediately available for
    /// new processes without requiring swapping or cache eviction.
    pub free_ram: f32,

    /// Amount of swap space currently in use, in megabytes.
    ///
    /// High swap usage may indicate memory pressure and can
    /// significantly impact system performance.
    pub used_swap: f32,
}

/// Structure for inserting disk information records into the database.
///
/// This structure represents disk usage information for a specific filesystem
/// at the time of system monitoring. Multiple disk records can be associated
/// with a single system information record through the `sysinfo_id` foreign key.
///
/// # Database Relationships
///
/// - `sysinfo_id`: Foreign key referencing the `sysinfo` table
/// - Each `SchemaSysInfo` record can have multiple associated `SchemaDiskInfo` records
///
/// # Storage Units
///
/// All size values are stored in megabytes for consistency and to avoid
/// integer overflow issues with very large storage devices.
///
/// # Examples
///
/// ```rust
/// use teus::monitor::schema::SchemaDiskInfo;
///
/// let disk_info = SchemaDiskInfo {
///     sysinfo_id: 1,
///     filesystem: "ext4".to_string(),
///     size: 1000000,      // 1TB in MB
///     used: 750000,       // 750GB in MB
///     available: 250000,  // 250GB in MB
///     used_percentage: 75,
///     mounted_path: "/".to_string(),
/// };
/// ```
#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = diskinfo)]
pub struct SchemaDiskInfo {
    /// Foreign key reference to the associated system information record.
    ///
    /// This links the disk information to a specific monitoring snapshot,
    /// allowing for historical tracking of disk usage over time.
    pub sysinfo_id: i32,

    /// Type of filesystem (e.g., "ext4", "ntfs", "xfs", "btrfs").
    ///
    /// This information helps identify the storage technology and
    /// can be useful for performance analysis and troubleshooting.
    pub filesystem: String,

    /// Total size of the filesystem in megabytes.
    ///
    /// This represents the total capacity of the storage device
    /// or partition, including space used by the filesystem metadata.
    pub size: i32,

    /// Amount of space currently used in megabytes.
    ///
    /// This includes all files, directories, and filesystem overhead,
    /// but may not account for reserved space depending on the filesystem.
    pub used: i32,

    /// Amount of space available for new data in megabytes.
    ///
    /// This is the space that can be immediately used for new files
    /// and may be less than (total - used) due to filesystem reservations.
    pub available: i32,

    /// Percentage of disk space currently in use.
    ///
    /// Range: 0 to 100, calculated as (used / total) * 100.
    /// Values above 90% typically indicate the need for cleanup or expansion.
    pub used_percentage: i32,

    /// Mount point or drive letter where the filesystem is accessible.
    ///
    /// Examples: "/", "/home", "/var", "C:", "D:"
    /// This helps identify which part of the system's storage hierarchy
    /// this disk information represents.
    pub mounted_path: String,
}

/// Structure for querying system information records from the database.
///
/// This structure is used when retrieving system monitoring data from the
/// database. It includes the database-generated ID field and can be used
/// for displaying historical monitoring data, generating reports, and
/// API responses.
///
/// # Usage Patterns
///
/// - Retrieving recent system performance data for dashboards
/// - Historical analysis and trend reporting
/// - API endpoints that return monitoring data to clients
/// - Data export and backup operations
///
/// # Examples
///
/// ```rust
/// use teus::monitor::schema::SysInfo;
/// use diesel::prelude::*;
///
/// // Query recent system information (pseudo-code)
/// // let recent_data: Vec<SysInfo> = sysinfo::table
/// //     .order(sysinfo::timestamp.desc())
/// //     .limit(10)
/// //     .load(&mut connection)?;
/// ```
#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = sysinfo)]
pub struct SysInfo {
    /// Database-generated unique identifier for this record.
    ///
    /// This is the primary key and is automatically assigned when
    /// the record is inserted into the database. Used for referencing
    /// this specific monitoring snapshot.
    #[diesel(column_name = id)]
    pub id: Option<i32>,

    /// Timestamp when this system information was collected.
    ///
    /// Stored in RFC3339 format for consistent parsing and timezone handling.
    pub timestamp: String,

    /// CPU usage percentage at the time of collection.
    ///
    /// Range: 0.0 to 100.0, representing the overall CPU utilization
    /// across all cores and threads.
    pub cpu_usage: f32,

    /// Amount of RAM currently in use, in megabytes.
    ///
    /// Active memory usage excluding cached and buffered memory.
    pub ram_usage: f32,

    /// Total amount of RAM available in the system, in megabytes.
    ///
    /// Physical memory capacity of the system.
    pub total_ram: f32,

    /// Amount of RAM currently free and available, in megabytes.
    ///
    /// Memory immediately available for allocation to new processes.
    pub free_ram: f32,

    /// Amount of swap space currently in use, in megabytes.
    ///
    /// High values may indicate memory pressure and performance issues.
    pub used_swap: f32,
}

/// Structure for querying disk information records from the database.
///
/// This structure represents stored disk usage information that can be
/// retrieved for historical analysis, reporting, and API responses.
/// It includes the database-generated ID and maintains the relationship
/// to its parent system information record.
///
/// # Relationships
///
/// Each `DiskInfo` record is linked to a `SysInfo` record through
/// the `sysinfo_id` foreign key, allowing for comprehensive system
/// monitoring data retrieval.
///
/// # Common Query Patterns
///
/// - Retrieving disk usage trends over time
/// - Finding disks approaching capacity limits
/// - Generating storage utilization reports
/// - Monitoring filesystem-specific usage patterns
///
/// # Examples
///
/// ```rust
/// use teus::monitor::schema::DiskInfo;
/// use diesel::prelude::*;
///
/// // Query disk info for high usage (pseudo-code)
/// // let high_usage_disks: Vec<DiskInfo> = diskinfo::table
/// //     .filter(diskinfo::used_percentage.gt(90))
/// //     .load(&mut connection)?;
/// ```
#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = diskinfo)]
pub struct DiskInfo {
    /// Database-generated unique identifier for this disk record.
    ///
    /// Primary key used for referencing this specific disk monitoring entry.
    #[diesel(column_name = id)]
    pub id: Option<i32>,

    /// Foreign key reference to the associated system information record.
    ///
    /// Links this disk information to a specific monitoring snapshot,
    /// enabling time-series analysis of disk usage.
    pub sysinfo_id: i32,

    /// Type of filesystem (e.g., "ext4", "ntfs", "xfs", "btrfs").
    ///
    /// Identifies the storage technology and formatting of this disk.
    pub filesystem: String,

    /// Total size of the filesystem in megabytes.
    ///
    /// Total capacity including filesystem overhead and reserved space.
    pub size: i32,

    /// Amount of space currently used in megabytes.
    ///
    /// Space occupied by files, directories, and filesystem metadata.
    pub used: i32,

    /// Amount of space available for new data in megabytes.
    ///
    /// Immediately usable space, may be less than (size - used)
    /// due to filesystem reservations and overhead.
    pub available: i32,

    /// Percentage of disk space currently in use.
    ///
    /// Range: 0 to 100, useful for quick assessment of storage pressure.
    /// Values above 90% typically require attention.
    pub used_percentage: i32,

    /// Mount point or drive letter where the filesystem is accessible.
    ///
    /// The path in the system's directory hierarchy where this
    /// storage device can be accessed (e.g., "/", "/home", "C:").
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
