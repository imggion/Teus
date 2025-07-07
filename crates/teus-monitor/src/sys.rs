use super::mutation;
use super::schema::{SchemaDiskInfo, SchemaSysInfo}; // Import the Diesel insertable structs
use teus_database::storage::Storage;
use teus_types::config::Config;
use chrono::Utc;
use teus_database::storage::TeuSQLiteConnection;
// use diesel::SqliteConnection; // Import SqliteConnection
use std::{thread, time::Duration}; // Import Mutex
use sysinfo::{Disks, MemoryRefreshKind, System};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiskInfo {
    pub available: usize,
    // Add disk-related fields here
    pub filesystem: String,
    pub mounted_path: String,
    pub size: usize,
    pub used: usize,
    pub used_percentage: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SysInfo {
    #[allow(dead_code)]
    pub id: i64,
    pub timestamp: String,
    pub cpu_usage: f64,
    pub ram_usage: f64,
    pub total_ram: f64,
    pub free_ram: f64,
    pub used_swap: f64,
    pub disks: Vec<DiskInfo>,
}

#[allow(dead_code)]
impl SysInfo {
    pub fn new(
        cpu_usage: f64,
        ram_usage: f64,
        total_ram: f64,
        free_ram: f64,
        used_swap: f64,
        disks: Vec<DiskInfo>,
    ) -> Self {
        Self {
            id: 0,
            timestamp: "".to_string(),
            cpu_usage,
            ram_usage,
            total_ram,
            free_ram,
            used_swap,
            disks,
        }
    }

    // Default constructor
    pub fn default() -> Self {
        Self {
            id: 0,
            timestamp: Utc::now().to_rfc3339(),
            cpu_usage: 0.0,
            ram_usage: 0.0,
            total_ram: 0.0,
            free_ram: 0.0,
            used_swap: 0.0,
            disks: vec![DiskInfo {
                filesystem: String::new(),
                size: 0,
                used: 0,
                available: 0,
                used_percentage: 0,
                mounted_path: String::new(),
            }],
        }
    }

    pub fn run_monitor(mut self, config: &Config) {
        let storage = match Storage::new(&config.database.path) {
            Ok(storage) => storage,
            Err(e) => {
                eprintln!("Failed to create storage: {}", e);
                return;
            }
        };

        // Get a mutable connection from the Arc<Mutex<>>
        let mut conn_guard = match storage.diesel_conn.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                eprintln!("Failed to acquire lock on DB connection: {}", poisoned);
                // Handle the poisoned mutex appropriately, maybe panic or return
                return;
            }
        };
        // Dereference the guard to get the &mut SqliteConnection
        let conn: &mut TeuSQLiteConnection = &mut *conn_guard;

        let mut sys = System::new_all();
        let disks_sysinfo = Disks::new_with_refreshed_list(); // Renamed to avoid conflict

        sys.refresh_all();
        sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

        self.total_ram = sys.total_memory() as f64;
        self.free_ram = sys.free_memory() as f64;
        self.used_swap = sys.used_swap() as f64;
        self.ram_usage = sys.used_memory() as f64; // Use used_memory for ram_usage

        thread::sleep(Duration::from_millis(250));
        sys.refresh_cpu_all();

        let cpu_count = sys.cpus().len();
        let total_cpu_usage: f64 = sys.cpus().iter().map(|cpu| cpu.cpu_usage() as f64).sum();
        self.cpu_usage = if cpu_count > 0 {
            total_cpu_usage / cpu_count as f64
        } else {
            0.0
        };

        self.timestamp = Utc::now().to_rfc3339(); // Ensure timestamp is current

        // Create the SchemaSysInfo struct for insertion
        let new_sys_info_to_insert = SchemaSysInfo {
            timestamp: self.timestamp,
            cpu_usage: self.cpu_usage as f32, // Cast f64 to f32
            ram_usage: self.ram_usage as f32, // Cast f64 to f32
            total_ram: self.total_ram as f32, // Cast f64 to f32
            free_ram: self.free_ram as f32,   // Cast f64 to f32
            used_swap: self.used_swap as f32, // Cast f64 to f32
        };

        // Insert system info using the SchemaSysInfo struct
        let sysinfo_id = match mutation::insert_sysinfo(conn, &new_sys_info_to_insert) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to insert system info: {}", e);
                // Drop the lock before returning
                drop(conn_guard);
                return;
            }
        };

        // Prepare disk info data for batch insertion
        let mut disk_infos_to_insert: Vec<SchemaDiskInfo> = Vec::new();
        for disk in disks_sysinfo.list() {
            let space_used = disk.total_space() - disk.available_space();
            // Calculate usage percentage correctly
            let usage_percentage = if disk.total_space() > 0 {
                (space_used as f64 / disk.total_space() as f64 * 100.0) as i32
            } else {
                0
            };

            let fs_name = disk.name().to_string_lossy().to_string();
            let mount_point = disk.mount_point().to_string_lossy().to_string();

            disk_infos_to_insert.push(SchemaDiskInfo {
                sysinfo_id, // Use the ID from the inserted sysinfo
                filesystem: fs_name,
                size: (disk.total_space() / 1024 / 1024) as i32, // Convert bytes to MB (adjust if needed) and cast usize to i32
                used: (space_used / 1024 / 1024) as i32, // Convert bytes to MB and cast usize to i32
                available: (disk.available_space() / 1024 / 1024) as i32, // Convert bytes to MB and cast usize to i32
                used_percentage: usage_percentage, // Use calculated percentage
                mounted_path: mount_point,
            });
        }

        // Insert disk info using the SchemaDiskInfo structs
        if !disk_infos_to_insert.is_empty() {
            if let Err(e) = mutation::insert_multiple_diskinfo(conn, &disk_infos_to_insert) {
                eprintln!("Failed to insert disk info batch: {}", e);
            }
        }
        // Lock is automatically dropped here when conn_guard goes out of scope
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teus_types::config::{Config, DatabaseConfig, Environment, MonitorConfig, ServerConfig};

    #[allow(dead_code)]
    fn create_test_config() -> Config {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                secret: "test_secret".to_string(),
                environment: Environment::Test,
            },
            database: DatabaseConfig {
                path: ":memory:".to_string(),
            },
            monitor: MonitorConfig { interval_secs: 60 },
        }
    }

    #[test]
    fn test_disk_info_creation() {
        let disk_info = DiskInfo {
            available: 1000,
            filesystem: "ext4".to_string(),
            mounted_path: "/".to_string(),
            size: 2000,
            used: 1000,
            used_percentage: 50,
        };

        assert_eq!(disk_info.available, 1000);
        assert_eq!(disk_info.filesystem, "ext4");
        assert_eq!(disk_info.mounted_path, "/");
        assert_eq!(disk_info.size, 2000);
        assert_eq!(disk_info.used, 1000);
        assert_eq!(disk_info.used_percentage, 50);
    }

    #[test]
    fn test_disk_info_clone() {
        let disk_info = DiskInfo {
            available: 500,
            filesystem: "ntfs".to_string(),
            mounted_path: "C:\\".to_string(),
            size: 1000,
            used: 500,
            used_percentage: 50,
        };

        let cloned = disk_info.clone();
        assert_eq!(disk_info.available, cloned.available);
        assert_eq!(disk_info.filesystem, cloned.filesystem);
        assert_eq!(disk_info.mounted_path, cloned.mounted_path);
        assert_eq!(disk_info.size, cloned.size);
        assert_eq!(disk_info.used, cloned.used);
        assert_eq!(disk_info.used_percentage, cloned.used_percentage);
    }

    #[test]
    fn test_sysinfo_new() {
        let disks = vec![DiskInfo {
            available: 1000,
            filesystem: "ext4".to_string(),
            mounted_path: "/".to_string(),
            size: 2000,
            used: 1000,
            used_percentage: 50,
        }];

        let sysinfo = SysInfo::new(25.5, 8000.0, 16000.0, 8000.0, 2000.0, disks.clone());

        assert_eq!(sysinfo.id, 0);
        assert_eq!(sysinfo.timestamp, "");
        assert_eq!(sysinfo.cpu_usage, 25.5);
        assert_eq!(sysinfo.ram_usage, 8000.0);
        assert_eq!(sysinfo.total_ram, 16000.0);
        assert_eq!(sysinfo.free_ram, 8000.0);
        assert_eq!(sysinfo.used_swap, 2000.0);
        assert_eq!(sysinfo.disks.len(), 1);
        assert_eq!(sysinfo.disks[0].filesystem, "ext4");
    }

    #[test]
    fn test_sysinfo_default() {
        let sysinfo = SysInfo::default();

        assert_eq!(sysinfo.id, 0);
        assert!(!sysinfo.timestamp.is_empty()); // Should have a timestamp
        assert_eq!(sysinfo.cpu_usage, 0.0);
        assert_eq!(sysinfo.ram_usage, 0.0);
        assert_eq!(sysinfo.total_ram, 0.0);
        assert_eq!(sysinfo.free_ram, 0.0);
        assert_eq!(sysinfo.used_swap, 0.0);
        assert_eq!(sysinfo.disks.len(), 1);
        assert_eq!(sysinfo.disks[0].filesystem, "");
        assert_eq!(sysinfo.disks[0].mounted_path, "");
        assert_eq!(sysinfo.disks[0].size, 0);
        assert_eq!(sysinfo.disks[0].used, 0);
        assert_eq!(sysinfo.disks[0].available, 0);
        assert_eq!(sysinfo.disks[0].used_percentage, 0);
    }

    #[test]
    fn test_sysinfo_clone() {
        let disks = vec![DiskInfo {
            available: 2000,
            filesystem: "btrfs".to_string(),
            mounted_path: "/home".to_string(),
            size: 4000,
            used: 2000,
            used_percentage: 50,
        }];

        let sysinfo = SysInfo::new(15.7, 4000.0, 8000.0, 4000.0, 1000.0, disks);

        let cloned = sysinfo.clone();
        assert_eq!(sysinfo.id, cloned.id);
        assert_eq!(sysinfo.timestamp, cloned.timestamp);
        assert_eq!(sysinfo.cpu_usage, cloned.cpu_usage);
        assert_eq!(sysinfo.ram_usage, cloned.ram_usage);
        assert_eq!(sysinfo.total_ram, cloned.total_ram);
        assert_eq!(sysinfo.free_ram, cloned.free_ram);
        assert_eq!(sysinfo.used_swap, cloned.used_swap);
        assert_eq!(sysinfo.disks.len(), cloned.disks.len());
        assert_eq!(sysinfo.disks[0].filesystem, cloned.disks[0].filesystem);
    }

    #[test]
    fn test_sysinfo_debug_format() {
        let sysinfo = SysInfo::default();
        let debug_str = format!("{:?}", sysinfo);
        assert!(debug_str.contains("SysInfo"));
        assert!(debug_str.contains("cpu_usage"));
        assert!(debug_str.contains("ram_usage"));
    }

    #[test]
    fn test_disk_info_debug_format() {
        let disk_info = DiskInfo {
            available: 1000,
            filesystem: "ext4".to_string(),
            mounted_path: "/".to_string(),
            size: 2000,
            used: 1000,
            used_percentage: 50,
        };

        let debug_str = format!("{:?}", disk_info);
        assert!(debug_str.contains("DiskInfo"));
        assert!(debug_str.contains("ext4"));
        assert!(debug_str.contains("1000"));
    }

    #[test]
    fn test_sysinfo_edge_values() {
        let disks = vec![];

        // Test with extreme values
        let sysinfo = SysInfo::new(
            100.0,           // Max CPU usage
            0.0,             // No RAM usage
            u64::MAX as f64, // Maximum possible RAM
            u64::MAX as f64, // Maximum free RAM
            0.0,             // No swap usage
            disks,
        );

        assert_eq!(sysinfo.cpu_usage, 100.0);
        assert_eq!(sysinfo.ram_usage, 0.0);
        assert_eq!(sysinfo.total_ram, u64::MAX as f64);
        assert_eq!(sysinfo.free_ram, u64::MAX as f64);
        assert_eq!(sysinfo.used_swap, 0.0);
        assert_eq!(sysinfo.disks.len(), 0);
    }

    #[test]
    fn test_disk_info_percentage_calculation() {
        // Test 100% usage
        let full_disk = DiskInfo {
            available: 0,
            filesystem: "ext4".to_string(),
            mounted_path: "/".to_string(),
            size: 1000,
            used: 1000,
            used_percentage: 100,
        };
        assert_eq!(full_disk.used_percentage, 100);

        // Test 0% usage
        let empty_disk = DiskInfo {
            available: 1000,
            filesystem: "ext4".to_string(),
            mounted_path: "/".to_string(),
            size: 1000,
            used: 0,
            used_percentage: 0,
        };
        assert_eq!(empty_disk.used_percentage, 0);
    }

    #[test]
    fn test_sysinfo_with_multiple_disks() {
        let disks = vec![
            DiskInfo {
                available: 1000,
                filesystem: "ext4".to_string(),
                mounted_path: "/".to_string(),
                size: 2000,
                used: 1000,
                used_percentage: 50,
            },
            DiskInfo {
                available: 500,
                filesystem: "ext4".to_string(),
                mounted_path: "/home".to_string(),
                size: 1000,
                used: 500,
                used_percentage: 50,
            },
        ];

        let sysinfo = SysInfo::new(30.0, 4000.0, 8000.0, 4000.0, 1000.0, disks);

        assert_eq!(sysinfo.disks.len(), 2);
        assert_eq!(sysinfo.disks[0].mounted_path, "/");
        assert_eq!(sysinfo.disks[1].mounted_path, "/home");
    }
}
