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
