use crate::{config::types::Config, monitor::storage::Storage};
use chrono::Utc;
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

        let conn = storage.clone().conn;
        let mut sys = System::new_all();
        let disks = Disks::new_with_refreshed_list();

        sys.refresh_all();

        // We don't want to update all memories information.
        sys.refresh_memory_specifics(MemoryRefreshKind::nothing().with_ram());

        self.total_ram = sys.total_memory() as f64;
        self.free_ram = sys.free_memory() as f64;
        self.used_swap = sys.used_swap() as f64;

        // TODO: Collect all cpus information and usage

        // Calculate the average CPU usage across all CPUs
        let cpu_count = sys.cpus().len();
        let total_cpu_usage: f64 = sys.cpus().iter().map(|cpu| cpu.cpu_usage() as f64).sum();

        // Set the average CPU usage as a percentage
        self.cpu_usage = if cpu_count > 0 {
            total_cpu_usage / cpu_count as f64
        } else {
            0.0
        };

        self.ram_usage = sys.used_memory() as f64;

        for disk in disks.list() {
            let space_used = disk.total_space() - disk.available_space();
            self.disks.push(DiskInfo {
                filesystem: disk.name().to_string_lossy().to_string(),
                size: disk.total_space() as usize,
                used: space_used as usize,
                available: disk.available_space() as usize,
                used_percentage: disk.usage().total_written_bytes as usize,
                mounted_path: disk.mount_point().to_string_lossy().to_string(),
            });
        }

        // Initialize database and handle potential errors
        if let Err(e) = storage.init_db(&conn) {
            eprintln!("Failed to initialize database: {}", e);
            return;
        }

        // Insert system info and handle potential errors
        let sysinfo_id = match storage.insert_sysinfo(&conn, &self) {
            Ok(id) => id,
            Err(e) => {
                eprintln!("Failed to insert system info: {}", e);
                return;
            }
        };

        // Insert disk info and handle potential errors
        for disk in self.disks.iter() {
            if let Err(e) = storage.insert_diskinfo(&conn, sysinfo_id, disk) {
                eprintln!("Failed to insert disk info: {}", e);
            }
        }
    }
}
