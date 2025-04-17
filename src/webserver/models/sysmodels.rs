use serde::Serialize;

/// Represents information about an IP address (private).
#[derive(serde::Serialize, Debug)]
pub struct IpInfo {
    /// The network interface name.
    pub interface: String,
    /// The IP address.
    pub addr: String,
    /// The subnet prefix length.
    pub prefix: u8,
}

/// Represents information about a MAC address (private).
#[derive(serde::Serialize, Debug)]
pub struct MACInfo {
    /// The network interface name.
    pub interface: String,
    /// The MAC address.
    pub mac: String,
}

/// Represents generic system information (private).
#[derive(serde::Serialize, Debug)]
pub struct GenericSysInfoResponse {
    /// The system's hostname.
    pub hostname: String,
    /// The operating system name.
    pub os: String,
    /// The system uptime.
    pub uptime: String,
    /// The kernel version.
    pub kernel_version: String,
    /// The primary IPv4 address.
    pub ipv4: String,
    /// List of network interfaces with IP information.
    pub networks: Vec<IpInfo>,
    /// List of MAC addresses.
    pub mac_addresses: Vec<MACInfo>,
}

impl GenericSysInfoResponse {}
impl Default for GenericSysInfoResponse {
    fn default() -> Self {
        Self {
            hostname: "No Info".to_string(),
            os: "No Info".to_string(),
            uptime: "No Info".to_string(),
            kernel_version: "No Info".to_string(),
            ipv4: "No Info".to_string(),
            networks: vec![],
            mac_addresses: vec![],
        }
    }
}

/// Represents the overall system information.
#[derive(Serialize)]
pub struct SysInfoResponse {
    /// The timestamp of the system information snapshot.
    pub timestamp: String,
    /// Percentage of CPU usage.
    pub cpu_usage: f32,
    /// Percentage of RAM usage.
    pub ram_usage: f32,
    /// Total RAM in the system (in bytes).
    pub total_ram: f32,
    /// Free RAM available (in bytes).
    pub free_ram: f32,
    /// Swap memory used (in bytes).
    pub used_swap: f32,
    /// List of disk information.
    pub disks: Vec<DiskInfoResponse>,
}

/// Represents information about a single disk.
#[derive(Serialize)]
pub struct DiskInfoResponse {
    /// The type of filesystem (e.g., ext4, NTFS).
    pub filesystem: String,
    /// The mount point of the disk.
    pub mount_point: String,
    /// Total space on the disk (in bytes).
    pub total_space: i32,
    /// Available space on the disk (in bytes).
    pub available_space: i32,
    /// Used space on the disk (in bytes).
    pub used_space: i32,
}
