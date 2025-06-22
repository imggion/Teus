//! System monitoring API response models.
//!
//! This module defines the response structures used by the Teus system monitoring
//! API endpoints. These structures provide a standardized format for returning
//! system information, network details, and resource utilization data to clients.

use serde::Serialize;

/// Network interface IP address information for API responses.
///
/// This structure represents IP address configuration for a specific network
/// interface on the monitored system. It includes both the address and subnet
/// information necessary for network analysis and monitoring.
///
/// # Usage
///
/// Typically used as part of system information responses to provide detailed
/// network configuration data. Multiple `IpInfo` instances may be included
/// for systems with multiple network interfaces.
///
/// # Examples
///
/// ```rust
/// use teus::webserver::models::sysmodels::IpInfo;
///
/// let ethernet_info = IpInfo {
///     interface: "eth0".to_string(),
///     addr: "192.168.1.100".to_string(),
///     prefix: 24,
/// };
///
/// let loopback_info = IpInfo {
///     interface: "lo".to_string(),
///     addr: "127.0.0.1".to_string(),
///     prefix: 8,
/// };
/// ```
///
/// # JSON Representation
///
/// ```json
/// {
///   "interface": "eth0",
///   "addr": "192.168.1.100",
///   "prefix": 24
/// }
/// ```
#[derive(serde::Serialize, Debug)]
pub struct IpInfo {
    /// The name of the network interface (e.g., "eth0", "wlan0", "lo").
    ///
    /// This identifies which physical or virtual network adapter
    /// the IP address is assigned to. Common interface names include:
    /// - "eth0", "eth1": Ethernet interfaces
    /// - "wlan0", "wlan1": Wireless interfaces  
    /// - "lo": Loopback interface
    /// - "docker0": Docker bridge interface
    pub interface: String,
    
    /// The IP address in string format.
    ///
    /// Can be either IPv4 (e.g., "192.168.1.100") or IPv6 format.
    /// The address represents the current network configuration
    /// for this interface.
    pub addr: String,
    
    /// The subnet prefix length (CIDR notation).
    ///
    /// Indicates the number of bits used for the network portion
    /// of the address. Common values:
    /// - 8: Class A networks (255.0.0.0)
    /// - 16: Class B networks (255.255.0.0)
    /// - 24: Class C networks (255.255.255.0)
    /// - 32: Host-specific routes
    pub prefix: u8,
}

/// Network interface MAC address information for API responses.
///
/// This structure represents the Media Access Control (MAC) address
/// configuration for a specific network interface. MAC addresses are
/// hardware identifiers that are unique to each network interface.
///
/// # Usage
///
/// Used in system information responses to provide network hardware
/// identification data. Essential for network troubleshooting,
/// asset management, and security monitoring.
///
/// # Examples
///
/// ```rust
/// use teus::webserver::models::sysmodels::MACInfo;
///
/// let mac_info = MACInfo {
///     interface: "eth0".to_string(),
///     mac: "aa:bb:cc:dd:ee:ff".to_string(),
/// };
/// ```
///
/// # JSON Representation
///
/// ```json
/// {
///   "interface": "eth0",
///   "mac": "aa:bb:cc:dd:ee:ff"
/// }
/// ```
#[derive(serde::Serialize, Debug)]
pub struct MACInfo {
    /// The name of the network interface.
    ///
    /// Corresponds to the same interface names used in `IpInfo`,
    /// allowing clients to correlate IP and MAC address information
    /// for each network adapter.
    pub interface: String,
    
    /// The MAC address in colon-separated hexadecimal format.
    ///
    /// Standard format is six groups of two hexadecimal digits
    /// separated by colons (e.g., "aa:bb:cc:dd:ee:ff").
    /// This is the unique hardware identifier for the network interface.
    pub mac: String,
}

/// Comprehensive system information response structure.
///
/// This structure provides a complete overview of the monitored system's
/// basic configuration and network setup. It's used by API endpoints
/// that need to return general system identification and network data.
///
/// # Use Cases
///
/// - System identification and inventory management
/// - Network configuration reporting
/// - Initial system assessment and setup verification
/// - Dashboard overview displays
///
/// # Network Information
///
/// The structure includes both summary (primary IPv4) and detailed
/// network information (all interfaces with IP and MAC addresses).
/// This allows clients to choose the appropriate level of detail
/// for their use case.
///
/// # Examples
///
/// ```rust
/// use teus::webserver::models::sysmodels::GenericSysInfoResponse;
///
/// let sys_info = GenericSysInfoResponse {
///     hostname: "server-01".to_string(),
///     os: "Ubuntu 22.04.3 LTS".to_string(),
///     uptime: "5 days, 14:32:10".to_string(),
///     kernel_version: "5.15.0-91-generic".to_string(),
///     ipv4: "192.168.1.100".to_string(),
///     networks: vec![/* network interfaces */],
///     mac_addresses: vec![/* MAC addresses */],
/// };
/// ```
///
/// # JSON Response Format
///
/// ```json
/// {
///   "hostname": "server-01",
///   "os": "Ubuntu 22.04.3 LTS",
///   "uptime": "5 days, 14:32:10",
///   "kernel_version": "5.15.0-91-generic",
///   "ipv4": "192.168.1.100",
///   "networks": [...],
///   "mac_addresses": [...]
/// }
/// ```
#[derive(serde::Serialize, Debug)]
pub struct GenericSysInfoResponse {
    /// The system's configured hostname.
    ///
    /// This is the name by which the system identifies itself
    /// on the network. Used for system identification and
    /// network management purposes.
    pub hostname: String,
    
    /// The operating system name and version.
    ///
    /// Provides detailed OS information including distribution
    /// name and version number. Examples:
    /// - "Ubuntu 22.04.3 LTS"
    /// - "CentOS Linux 8.4.2105"
    /// - "Windows Server 2019"
    pub os: String,
    
    /// Human-readable system uptime information.
    ///
    /// Indicates how long the system has been running since
    /// the last boot. Format may vary but typically includes
    /// days, hours, minutes, and seconds.
    pub uptime: String,
    
    /// The kernel version string.
    ///
    /// Provides the version of the operating system kernel.
    /// Important for compatibility checking and security
    /// vulnerability assessment.
    pub kernel_version: String,
    
    /// The primary IPv4 address of the system.
    ///
    /// This is typically the main network address used for
    /// external communication. Useful for quick identification
    /// and network connectivity verification.
    pub ipv4: String,
    
    /// Detailed information about all network interfaces.
    ///
    /// Contains IP address configuration for each network
    /// interface on the system, including physical and
    /// virtual interfaces.
    pub networks: Vec<IpInfo>,
    
    /// MAC address information for all network interfaces.
    ///
    /// Provides hardware identifiers for network interfaces,
    /// useful for asset tracking and network security.
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

/// Real-time system performance and resource utilization response.
///
/// This structure represents a complete snapshot of system resource
/// usage at a specific point in time. It's the primary response format
/// for monitoring API endpoints that provide current system performance data.
///
/// # Monitoring Data
///
/// Includes comprehensive resource utilization metrics:
/// - CPU usage percentage
/// - Memory usage and availability
/// - Swap space utilization
/// - Storage usage across all mounted filesystems
///
/// # Time Series Data
///
/// The timestamp field enables this structure to be used for time-series
/// monitoring, allowing clients to track performance trends over time.
///
/// # API Endpoints
///
/// Typically returned by:
/// - `/api/system/current` - Current system status
/// - `/api/monitoring/snapshot` - Performance snapshot
/// - Real-time monitoring WebSocket endpoints
///
/// # Examples
///
/// ```rust
/// use teus::webserver::models::sysmodels::{SysInfoResponse, DiskInfoResponse};
/// use chrono::Utc;
///
/// let response = SysInfoResponse {
///     timestamp: Utc::now().to_rfc3339(),
///     cpu_usage: 35.2,
///     ram_usage: 8192.0,
///     total_ram: 16384.0,
///     free_ram: 8192.0,
///     used_swap: 0.0,
///     disks: vec![/* disk information */],
/// };
/// ```
///
/// # JSON Response Format
///
/// ```json
/// {
///   "timestamp": "2024-01-15T10:30:00Z",
///   "cpu_usage": 35.2,
///   "ram_usage": 8192.0,
///   "total_ram": 16384.0,
///   "free_ram": 8192.0,
///   "used_swap": 0.0,
///   "disks": [...]
/// }
/// ```
#[derive(Serialize)]
pub struct SysInfoResponse {
    /// ISO 8601 timestamp when this data was collected.
    ///
    /// Typically in RFC3339 format (e.g., "2024-01-15T10:30:00Z").
    /// Essential for time-series analysis and determining data freshness.
    pub timestamp: String,
    
    /// Current CPU usage as a percentage (0.0 to 100.0).
    ///
    /// Represents the overall CPU utilization across all cores
    /// and threads. Values approaching 100% indicate high
    /// computational load that may affect system responsiveness.
    pub cpu_usage: f32,
    
    /// Current RAM usage in megabytes.
    ///
    /// This represents the amount of physical memory currently
    /// allocated to running processes, excluding cached and
    /// buffered memory that can be quickly reclaimed.
    pub ram_usage: f32,
    
    /// Total available RAM in the system, in megabytes.
    ///
    /// This is the total physical memory capacity and should
    /// remain constant unless hardware changes occur. Used
    /// to calculate usage percentages and available capacity.
    pub total_ram: f32,
    
    /// Amount of RAM currently free and immediately available, in megabytes.
    ///
    /// This represents memory that can be immediately allocated
    /// to new processes without requiring swap operations or
    /// cache eviction. Critical for assessing memory pressure.
    pub free_ram: f32,
    
    /// Current swap space usage in megabytes.
    ///
    /// High swap usage may indicate memory pressure and can
    /// significantly impact system performance. Values should
    /// typically remain low in well-configured systems.
    pub used_swap: f32,
    
    /// Storage utilization information for all mounted filesystems.
    ///
    /// Provides detailed disk usage data for each storage device
    /// or partition mounted on the system. Critical for storage
    /// capacity planning and preventing disk space exhaustion.
    pub disks: Vec<DiskInfoResponse>,
}

/// Storage device utilization information for API responses.
///
/// This structure represents the storage usage details for a single
/// filesystem or storage device. It provides comprehensive information
/// about capacity, utilization, and availability for storage monitoring.
///
/// # Storage Metrics
///
/// Includes all essential storage metrics:
/// - Total capacity and current usage
/// - Available space for new data
/// - Filesystem type and mount location
///
/// # Monitoring Applications
///
/// Used for:
/// - Storage capacity planning and alerts
/// - Disk space utilization tracking
/// - Filesystem performance monitoring
/// - Storage infrastructure management
///
/// # Examples
///
/// ```rust
/// use teus::webserver::models::sysmodels::DiskInfoResponse;
///
/// let root_disk = DiskInfoResponse {
///     filesystem: "ext4".to_string(),
///     mount_point: "/".to_string(),
///     total_space: 1000000,    // 1TB in MB
///     available_space: 750000, // 750GB available
///     used_space: 250000,      // 250GB used
/// };
///
/// let data_disk = DiskInfoResponse {
///     filesystem: "xfs".to_string(),
///     mount_point: "/data".to_string(),
///     total_space: 2000000,    // 2TB in MB
///     available_space: 1800000, // 1.8TB available
///     used_space: 200000,      // 200GB used
/// };
/// ```
///
/// # JSON Response Format
///
/// ```json
/// {
///   "filesystem": "ext4",
///   "mount_point": "/",
///   "total_space": 1000000,
///   "available_space": 750000,
///   "used_space": 250000
/// }
/// ```
#[derive(Serialize)]
pub struct DiskInfoResponse {
    /// The filesystem type (e.g., "ext4", "xfs", "ntfs", "btrfs").
    ///
    /// Identifies the filesystem format and technology used
    /// on this storage device. Important for understanding
    /// performance characteristics and supported features.
    pub filesystem: String,
    
    /// The mount point or drive path where the filesystem is accessible.
    ///
    /// Examples:
    /// - "/" - Root filesystem on Unix-like systems
    /// - "/home" - User data partition
    /// - "/var" - Variable data partition
    /// - "C:" - Windows system drive
    /// - "/mnt/data" - Mounted data drive
    pub mount_point: String,
    
    /// Total storage capacity in megabytes.
    ///
    /// This represents the complete size of the filesystem,
    /// including space used by the filesystem metadata and
    /// any reserved blocks. Used for capacity planning calculations.
    pub total_space: i32,
    
    /// Currently available space for new data, in megabytes.
    ///
    /// This is the space that can be immediately used for new
    /// files and directories. May be less than (total - used)
    /// due to filesystem reservations and overhead.
    pub available_space: i32,
    
    /// Currently used storage space in megabytes.
    ///
    /// Represents the space occupied by files, directories,
    /// and filesystem metadata. Used to calculate utilization
    /// percentages and remaining capacity.
    pub used_space: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_info_creation() {
        let ip_info = IpInfo {
            interface: "eth0".to_string(),
            addr: "192.168.1.10".to_string(),
            prefix: 24,
        };

        assert_eq!(ip_info.interface, "eth0");
        assert_eq!(ip_info.addr, "192.168.1.10");
        assert_eq!(ip_info.prefix, 24);
    }

    #[test]
    fn test_ip_info_serialization() {
        let ip_info = IpInfo {
            interface: "wlan0".to_string(),
            addr: "10.0.0.5".to_string(),
            prefix: 16,
        };

        let serialized = serde_json::to_string(&ip_info).unwrap();
        assert!(serialized.contains("\"interface\":\"wlan0\""));
        assert!(serialized.contains("\"addr\":\"10.0.0.5\""));
        assert!(serialized.contains("\"prefix\":16"));
    }

    #[test]
    fn test_mac_info_creation() {
        let mac_info = MACInfo {
            interface: "eth0".to_string(),
            mac: "aa:bb:cc:dd:ee:ff".to_string(),
        };

        assert_eq!(mac_info.interface, "eth0");
        assert_eq!(mac_info.mac, "aa:bb:cc:dd:ee:ff");
    }

    #[test]
    fn test_mac_info_serialization() {
        let mac_info = MACInfo {
            interface: "wlan0".to_string(),
            mac: "11:22:33:44:55:66".to_string(),
        };

        let serialized = serde_json::to_string(&mac_info).unwrap();
        assert!(serialized.contains("\"interface\":\"wlan0\""));
        assert!(serialized.contains("\"mac\":\"11:22:33:44:55:66\""));
    }

    #[test]
    fn test_generic_sys_info_default() {
        let sys_info = GenericSysInfoResponse::default();

        assert_eq!(sys_info.hostname, "No Info");
        assert_eq!(sys_info.os, "No Info");
        assert_eq!(sys_info.uptime, "No Info");
        assert_eq!(sys_info.kernel_version, "No Info");
        assert_eq!(sys_info.ipv4, "No Info");
        assert!(sys_info.networks.is_empty());
        assert!(sys_info.mac_addresses.is_empty());
    }

    #[test]
    fn test_generic_sys_info_creation() {
        let networks = vec![
            IpInfo {
                interface: "eth0".to_string(),
                addr: "192.168.1.10".to_string(),
                prefix: 24,
            },
            IpInfo {
                interface: "lo".to_string(),
                addr: "127.0.0.1".to_string(),
                prefix: 8,
            },
        ];

        let mac_addresses = vec![
            MACInfo {
                interface: "eth0".to_string(),
                mac: "aa:bb:cc:dd:ee:ff".to_string(),
            },
        ];

        let sys_info = GenericSysInfoResponse {
            hostname: "test-machine".to_string(),
            os: "Linux".to_string(),
            uptime: "5 days".to_string(),
            kernel_version: "5.4.0".to_string(),
            ipv4: "192.168.1.10".to_string(),
            networks,
            mac_addresses,
        };

        assert_eq!(sys_info.hostname, "test-machine");
        assert_eq!(sys_info.os, "Linux");
        assert_eq!(sys_info.uptime, "5 days");
        assert_eq!(sys_info.kernel_version, "5.4.0");
        assert_eq!(sys_info.ipv4, "192.168.1.10");
        assert_eq!(sys_info.networks.len(), 2);
        assert_eq!(sys_info.mac_addresses.len(), 1);
    }

    #[test]
    fn test_generic_sys_info_serialization() {
        let sys_info = GenericSysInfoResponse {
            hostname: "server1".to_string(),
            os: "Ubuntu 20.04".to_string(),
            uptime: "10 hours".to_string(),
            kernel_version: "5.4.0-42".to_string(),
            ipv4: "10.0.0.1".to_string(),
            networks: vec![],
            mac_addresses: vec![],
        };

        let serialized = serde_json::to_string(&sys_info).unwrap();
        assert!(serialized.contains("\"hostname\":\"server1\""));
        assert!(serialized.contains("\"os\":\"Ubuntu 20.04\""));
        assert!(serialized.contains("\"uptime\":\"10 hours\""));
        assert!(serialized.contains("\"kernel_version\":\"5.4.0-42\""));
        assert!(serialized.contains("\"ipv4\":\"10.0.0.1\""));
    }

    #[test]
    fn test_disk_info_response_creation() {
        let disk_info = DiskInfoResponse {
            filesystem: "ext4".to_string(),
            mount_point: "/".to_string(),
            total_space: 1000000,
            available_space: 500000,
            used_space: 500000,
        };

        assert_eq!(disk_info.filesystem, "ext4");
        assert_eq!(disk_info.mount_point, "/");
        assert_eq!(disk_info.total_space, 1000000);
        assert_eq!(disk_info.available_space, 500000);
        assert_eq!(disk_info.used_space, 500000);
    }

    #[test]
    fn test_disk_info_response_serialization() {
        let disk_info = DiskInfoResponse {
            filesystem: "ntfs".to_string(),
            mount_point: "C:\\".to_string(),
            total_space: 2000000,
            available_space: 1000000,
            used_space: 1000000,
        };

        let serialized = serde_json::to_string(&disk_info).unwrap();
        assert!(serialized.contains("\"filesystem\":\"ntfs\""));
        assert!(serialized.contains("\"mount_point\":\"C:\\\\\""));
        assert!(serialized.contains("\"total_space\":2000000"));
        assert!(serialized.contains("\"available_space\":1000000"));
        assert!(serialized.contains("\"used_space\":1000000"));
    }

    #[test]
    fn test_sys_info_response_creation() {
        let disks = vec![
            DiskInfoResponse {
                filesystem: "ext4".to_string(),
                mount_point: "/".to_string(),
                total_space: 1000000,
                available_space: 600000,
                used_space: 400000,
            },
            DiskInfoResponse {
                filesystem: "ext4".to_string(),
                mount_point: "/home".to_string(),
                total_space: 2000000,
                available_space: 1500000,
                used_space: 500000,
            },
        ];

        let sys_info = SysInfoResponse {
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            cpu_usage: 25.5,
            ram_usage: 4096.0,
            total_ram: 8192.0,
            free_ram: 4096.0,
            used_swap: 512.0,
            disks,
        };

        assert_eq!(sys_info.timestamp, "2024-01-01T00:00:00Z");
        assert_eq!(sys_info.cpu_usage, 25.5);
        assert_eq!(sys_info.ram_usage, 4096.0);
        assert_eq!(sys_info.total_ram, 8192.0);
        assert_eq!(sys_info.free_ram, 4096.0);
        assert_eq!(sys_info.used_swap, 512.0);
        assert_eq!(sys_info.disks.len(), 2);
        assert_eq!(sys_info.disks[0].mount_point, "/");
        assert_eq!(sys_info.disks[1].mount_point, "/home");
    }

    #[test]
    fn test_sys_info_response_serialization() {
        let sys_info = SysInfoResponse {
            timestamp: "2024-01-01T12:00:00Z".to_string(),
            cpu_usage: 50.0,
            ram_usage: 2048.0,
            total_ram: 4096.0,
            free_ram: 2048.0,
            used_swap: 0.0,
            disks: vec![],
        };

        let serialized = serde_json::to_string(&sys_info).unwrap();
        assert!(serialized.contains("\"timestamp\":\"2024-01-01T12:00:00Z\""));
        assert!(serialized.contains("\"cpu_usage\":50"));
        assert!(serialized.contains("\"ram_usage\":2048"));
        assert!(serialized.contains("\"total_ram\":4096"));
        assert!(serialized.contains("\"free_ram\":2048"));
        assert!(serialized.contains("\"used_swap\":0"));
        assert!(serialized.contains("\"disks\":[]"));
    }

    #[test]
    fn test_debug_formatting() {
        let ip_info = IpInfo {
            interface: "eth0".to_string(),
            addr: "192.168.1.1".to_string(),
            prefix: 24,
        };

        let debug_str = format!("{:?}", ip_info);
        assert!(debug_str.contains("IpInfo"));
        assert!(debug_str.contains("eth0"));
        assert!(debug_str.contains("192.168.1.1"));
        assert!(debug_str.contains("24"));

        let mac_info = MACInfo {
            interface: "eth0".to_string(),
            mac: "00:11:22:33:44:55".to_string(),
        };

        let debug_str = format!("{:?}", mac_info);
        assert!(debug_str.contains("MACInfo"));
        assert!(debug_str.contains("eth0"));
        assert!(debug_str.contains("00:11:22:33:44:55"));
    }

    #[test]
    fn test_edge_cases() {
        // Test with empty strings
        let ip_info = IpInfo {
            interface: "".to_string(),
            addr: "".to_string(),
            prefix: 0,
        };
        assert_eq!(ip_info.interface, "");
        assert_eq!(ip_info.addr, "");
        assert_eq!(ip_info.prefix, 0);

        // Test with maximum prefix
        let ip_info_max = IpInfo {
            interface: "test".to_string(),
            addr: "255.255.255.255".to_string(),
            prefix: 32,
        };
        assert_eq!(ip_info_max.prefix, 32);

        // Test disk with zero values
        let disk_info = DiskInfoResponse {
            filesystem: "tmpfs".to_string(),
            mount_point: "/tmp".to_string(),
            total_space: 0,
            available_space: 0,
            used_space: 0,
        };
        assert_eq!(disk_info.total_space, 0);
        assert_eq!(disk_info.available_space, 0);
        assert_eq!(disk_info.used_space, 0);
    }
}
