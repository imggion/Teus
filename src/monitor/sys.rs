use super::mutation;
use super::schema::{SchemaDiskInfo, SchemaSysInfo}; // Import the Diesel insertable structs
use crate::{config::types::Config, monitor::storage::Storage};
use chrono::Utc;
use diesel::SqliteConnection; // Import SqliteConnection
use std::{sync::Mutex, thread, time::Duration}; // Import Mutex
use sysinfo::{Disks, MemoryRefreshKind, System};

#[derive(Clone, Debug)]
pub struct DiskInfo {
    // Add disk-related fields here
    pub filesystem: String,
    pub size: usize,
    pub used: usize,
    pub available: usize,
    pub used_percentage: usize,
    pub mounted_path: String,
}

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
        let conn: &mut SqliteConnection = &mut *conn_guard;

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

        // --- Prepare data for Diesel insertion ---
        self.timestamp = Utc::now().to_rfc3339(); // Ensure timestamp is current

        // Create the SchemaSysInfo struct for insertion
        // NOTE: Assuming user_id = 1 for now. This should be dynamic in a real app.
        let new_sys_info_to_insert = SchemaSysInfo {
            timestamp: self.timestamp,
            cpu_usage: self.cpu_usage as f32, // Cast f64 to f32
            ram_usage: self.ram_usage as f32, // Cast f64 to f32
            total_ram: self.total_ram as f32, // Cast f64 to f32
            free_ram: self.free_ram as f32,   // Cast f64 to f32
            used_swap: self.used_swap as f32, // Cast f64 to f32
                                              // user_id: 1,                       // Placeholder user_id
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
                sysinfo_id: sysinfo_id, // Use the ID from the inserted sysinfo
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
