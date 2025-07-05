use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Response from Container Inspect operation
/// Based on Docker Engine API v1.50 ContainerInspectResponse
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ContainerInspectResponse {
    /// The ID of this container as a 128-bit (64-character) hexadecimal string (32 bytes)
    #[serde(rename = "Id")]
    pub id: String,

    /// Date and time at which the container was created, formatted in RFC 3339 format with nano-seconds
    #[serde(rename = "Created")]
    pub created: Option<String>,

    /// The path to the command being run
    #[serde(rename = "Path")]
    pub path: String,

    /// The arguments to the command being run
    #[serde(rename = "Args")]
    pub args: Vec<String>,

    /// Container state information
    #[serde(rename = "State")]
    pub state: Option<ContainerState>,

    /// The ID (digest) of the image that this container was created from
    #[serde(rename = "Image")]
    pub image: String,

    /// Location of the /etc/resolv.conf generated for the container on the host
    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: String,

    /// Location of the /etc/hostname generated for the container on the host
    #[serde(rename = "HostnamePath")]
    pub hostname_path: String,

    /// Location of the /etc/hosts generated for the container on the host
    #[serde(rename = "HostsPath")]
    pub hosts_path: String,

    /// Location of the file used to buffer the container's logs
    #[serde(rename = "LogPath")]
    pub log_path: Option<String>,

    /// The name associated with this container
    #[serde(rename = "Name")]
    pub name: String,

    /// Number of times the container was restarted since it was created
    #[serde(rename = "RestartCount")]
    pub restart_count: i64,

    /// The storage-driver used for the container's filesystem
    #[serde(rename = "Driver")]
    pub driver: String,

    /// The platform (operating system) for which the container was created
    #[serde(rename = "Platform")]
    pub platform: String,

    /// OCI descriptor of the platform-specific manifest of the image
    #[serde(rename = "ImageManifestDescriptor")]
    pub image_manifest_descriptor: Option<OciDescriptor>,

    /// SELinux mount label set for the container
    #[serde(rename = "MountLabel")]
    pub mount_label: String,

    /// SELinux process label set for the container
    #[serde(rename = "ProcessLabel")]
    pub process_label: String,

    /// The AppArmor profile set for the container
    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: String,

    /// IDs of exec instances that are running in the container
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Option<Vec<String>>,

    /// Host configuration for this container
    #[serde(rename = "HostConfig")]
    pub host_config: Option<HostConfig>,

    /// Information about the container's graph driver
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<DriverData>,

    /// The size of files that have been created or changed by this container
    #[serde(rename = "SizeRw")]
    pub size_rw: Option<i64>,

    /// The total size of all files in the read-only layers from the image
    #[serde(rename = "SizeRootFs")]
    pub size_root_fs: Option<i64>,

    /// List of mounts used by the container
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<MountPoint>>,

    /// Container configuration
    #[serde(rename = "Config")]
    pub config: Option<ContainerConfig>,

    /// Network settings for the container
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<NetworkSettings>,
}

/// ContainerState stores container's running state
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContainerState {
    /// String representation of the container state
    #[serde(rename = "Status")]
    pub status: Option<String>,

    /// Whether this container is running
    #[serde(rename = "Running")]
    pub running: Option<bool>,

    /// Whether this container is paused
    #[serde(rename = "Paused")]
    pub paused: Option<bool>,

    /// Whether this container is restarting
    #[serde(rename = "Restarting")]
    pub restarting: Option<bool>,

    /// Whether a process within this container has been killed because it ran out of memory
    #[serde(rename = "OOMKilled")]
    pub oom_killed: Option<bool>,

    /// Whether the container is dead
    #[serde(rename = "Dead")]
    pub dead: Option<bool>,

    /// The process ID of this container
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,

    /// The last exit code of this container
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,

    /// Error message if container failed
    #[serde(rename = "Error")]
    pub error: Option<String>,

    /// The time when this container was last started
    #[serde(rename = "StartedAt")]
    pub started_at: Option<String>,

    /// The time when this container last exited
    #[serde(rename = "FinishedAt")]
    pub finished_at: Option<String>,

    /// Health check information
    #[serde(rename = "Health")]
    pub health: Option<Health>,
}

/// Health check information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Health {
    /// Health status
    #[serde(rename = "Status")]
    pub status: Option<String>,

    /// Number of consecutive failures
    #[serde(rename = "FailingStreak")]
    pub failing_streak: Option<i64>,

    /// Health check logs
    #[serde(rename = "Log")]
    pub log: Option<Vec<HealthLog>>,
}

/// Health check log entry
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HealthLog {
    /// Start time of the health check
    #[serde(rename = "Start")]
    pub start: Option<String>,

    /// End time of the health check
    #[serde(rename = "End")]
    pub end: Option<String>,

    /// Exit code of the health check
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,

    /// Output of the health check
    #[serde(rename = "Output")]
    pub output: Option<String>,
}

/// OCI descriptor for image manifests
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OciDescriptor {
    /// Media type of the descriptor
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,

    /// Digest of the content
    #[serde(rename = "digest")]
    pub digest: Option<String>,

    /// Size in bytes
    #[serde(rename = "size")]
    pub size: Option<i64>,

    /// List of URLs
    #[serde(rename = "urls")]
    pub urls: Option<Vec<String>>,

    /// Annotations
    #[serde(rename = "annotations")]
    pub annotations: Option<HashMap<String, String>>,

    /// Data
    #[serde(rename = "data")]
    pub data: Option<String>,

    /// Platform information
    #[serde(rename = "platform")]
    pub platform: Option<Platform>,

    /// Artifact type
    #[serde(rename = "artifactType")]
    pub artifact_type: Option<String>,
}

/// Platform information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Platform {
    /// Architecture
    #[serde(rename = "architecture")]
    pub architecture: Option<String>,

    /// Operating system
    #[serde(rename = "os")]
    pub os: Option<String>,

    /// OS version
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,

    /// OS features
    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,

    /// Architecture variant
    #[serde(rename = "variant")]
    pub variant: Option<String>,
}

/// Container host configuration
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HostConfig {
    /// Volume bindings for this container
    #[serde(rename = "Binds")]
    pub binds: Option<Vec<String>>,

    /// Path to a file where the container ID is written
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: Option<String>,

    /// The logging configuration for this container
    #[serde(rename = "LogConfig")]
    pub log_config: Option<LogConfig>,

    /// Network mode to use for this container
    #[serde(rename = "NetworkMode")]
    pub network_mode: Option<String>,

    /// Port bindings
    #[serde(rename = "PortBindings")]
    pub port_bindings: Option<HashMap<String, Option<Vec<PortBinding>>>>,

    /// Restart policy
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<RestartPolicy>,

    /// Automatically remove the container when the container's process exits
    #[serde(rename = "AutoRemove")]
    pub auto_remove: Option<bool>,

    /// Driver that this container uses to mount volumes
    #[serde(rename = "VolumeDriver")]
    pub volume_driver: Option<String>,

    /// List of volumes to inherit from another container
    #[serde(rename = "VolumesFrom")]
    pub volumes_from: Option<Vec<String>>,

    /// Specification for mounts to be added to the container
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<Mount>>,

    /// Initial console size, as an [height, width] array
    #[serde(rename = "ConsoleSize")]
    pub console_size: Option<Vec<i64>>,

    /// Arbitrary non-identifying metadata attached to container
    #[serde(rename = "Annotations")]
    pub annotations: Option<HashMap<String, String>>,

    /// CPU shares (relative weight)
    #[serde(rename = "CpuShares")]
    pub cpu_shares: Option<i64>,

    /// Memory limit in bytes
    #[serde(rename = "Memory")]
    pub memory: Option<i64>,

    /// Total memory limit (memory + swap)
    #[serde(rename = "MemorySwap")]
    pub memory_swap: Option<i64>,

    /// Memory soft limit in bytes
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<i64>,

    /// Tune a container's memory swappiness behavior
    #[serde(rename = "MemorySwappiness")]
    pub memory_swappiness: Option<i64>,

    /// CPU quota in units of 10^-9 CPUs
    #[serde(rename = "NanoCpus")]
    pub nano_cpus: Option<i64>,

    /// Cgroup parent
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: Option<String>,

    /// Block IO weight (relative weight)
    #[serde(rename = "BlkioWeight")]
    pub blkio_weight: Option<i64>,

    /// Block IO weight device
    #[serde(rename = "BlkioWeightDevice")]
    pub blkio_weight_device: Option<Vec<ThrottleDevice>>,

    /// Limit read rate (bytes per second) from a device
    #[serde(rename = "BlkioDeviceReadBps")]
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (bytes per second) to a device
    #[serde(rename = "BlkioDeviceWriteBps")]
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,

    /// Limit read rate (IO per second) from a device
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,

    /// Limit write rate (IO per second) to a device
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,

    /// The length of a CPU period in microseconds
    #[serde(rename = "CpuPeriod")]
    pub cpu_period: Option<i64>,

    /// Microseconds of CPU time that the container can get in a CPU period
    #[serde(rename = "CpuQuota")]
    pub cpu_quota: Option<i64>,

    /// The length of a CPU real-time period in microseconds
    #[serde(rename = "CpuRealtimePeriod")]
    pub cpu_realtime_period: Option<i64>,

    /// The length of a CPU real-time runtime in microseconds
    #[serde(rename = "CpuRealtimeRuntime")]
    pub cpu_realtime_runtime: Option<i64>,

    /// CPUs in which to allow execution (0-3, 0,1)
    #[serde(rename = "CpusetCpus")]
    pub cpuset_cpus: Option<String>,

    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1)
    #[serde(rename = "CpusetMems")]
    pub cpuset_mems: Option<String>,

    /// A list of devices to add to the container
    #[serde(rename = "Devices")]
    pub devices: Option<Vec<DeviceMapping>>,

    /// A list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules")]
    pub device_cgroup_rules: Option<Vec<String>>,

    /// A list of requests for devices to be sent to device drivers
    #[serde(rename = "DeviceRequests")]
    pub device_requests: Option<Vec<DeviceRequest>>,

    /// Kernel memory TCP limit in bytes
    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: Option<i64>,

    /// Disable OOM Killer for the container
    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: Option<bool>,

    /// Run an init inside the container that forwards signals and reaps processes
    #[serde(rename = "Init")]
    pub init: Option<bool>,

    /// Tune a container's PIDs limit
    #[serde(rename = "PidsLimit")]
    pub pids_limit: Option<i64>,

    /// A list of resource limits to set in the container
    #[serde(rename = "Ulimits")]
    pub ulimits: Option<Vec<Ulimit>>,

    /// The number of usable CPUs (Windows only)
    #[serde(rename = "CpuCount")]
    pub cpu_count: Option<i64>,

    /// The usable percentage of the available CPUs (Windows only)
    #[serde(rename = "CpuPercent")]
    pub cpu_percent: Option<i64>,

    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: Option<i64>,

    /// Maximum IO in bytes per second for the container system drive (Windows only)
    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<i64>,

    /// A list of kernel capabilities to add to the container
    #[serde(rename = "CapAdd")]
    pub cap_add: Option<Vec<String>>,

    /// A list of kernel capabilities to drop from the container
    #[serde(rename = "CapDrop")]
    pub cap_drop: Option<Vec<String>>,

    /// Cgroup namespace mode for the container
    #[serde(rename = "CgroupnsMode")]
    pub cgroupns_mode: Option<String>,

    /// A list of DNS servers for the container to use
    #[serde(rename = "Dns")]
    pub dns: Option<Vec<String>>,

    /// A list of DNS options
    #[serde(rename = "DnsOptions")]
    pub dns_options: Option<Vec<String>>,

    /// A list of DNS search domains
    #[serde(rename = "DnsSearch")]
    pub dns_search: Option<Vec<String>>,

    /// A list of hostnames/IP mappings to add to the container's /etc/hosts file
    #[serde(rename = "ExtraHosts")]
    pub extra_hosts: Option<Vec<String>>,

    /// A list of additional groups that the container process will run as
    #[serde(rename = "GroupAdd")]
    pub group_add: Option<Vec<String>>,

    /// IPC sharing mode for the container
    #[serde(rename = "IpcMode")]
    pub ipc_mode: Option<String>,

    /// Cgroup to use for the container
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,

    /// A list of links for the container in the form container_name:alias
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,

    /// An integer value containing the score given to the container in order to tune OOM killer preferences
    #[serde(rename = "OomScoreAdj")]
    pub oom_score_adj: Option<i64>,

    /// Set the PID (Process) Namespace mode for the container
    #[serde(rename = "PidMode")]
    pub pid_mode: Option<String>,

    /// Gives the container full access to the host
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,

    /// Allocates an ephemeral host port for all of a container's exposed ports
    #[serde(rename = "PublishAllPorts")]
    pub publish_all_ports: Option<bool>,

    /// Mount the container's root filesystem as read only
    #[serde(rename = "ReadonlyRootfs")]
    pub readonly_rootfs: Option<bool>,

    /// A list of string values to customize labels for MLS systems
    #[serde(rename = "SecurityOpt")]
    pub security_opt: Option<Vec<String>>,

    /// Storage driver options per container
    #[serde(rename = "StorageOpt")]
    pub storage_opt: Option<HashMap<String, String>>,

    /// A map of container directories which should be replaced by tmpfs mounts
    #[serde(rename = "Tmpfs")]
    pub tmpfs: Option<HashMap<String, String>>,

    /// UTS namespace to use for the container
    #[serde(rename = "UTSMode")]
    pub uts_mode: Option<String>,

    /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled
    #[serde(rename = "UsernsMode")]
    pub userns_mode: Option<String>,

    /// Size of /dev/shm in bytes
    #[serde(rename = "ShmSize")]
    pub shm_size: Option<i64>,

    /// A list of kernel parameters (sysctls) to set in the container
    #[serde(rename = "Sysctls")]
    pub sysctls: Option<HashMap<String, String>>,

    /// Runtime to use with this container
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,

    /// Isolation technology of the container
    #[serde(rename = "Isolation")]
    pub isolation: Option<String>,

    /// The list of paths to be masked inside the container (this overrides the default set of paths)
    #[serde(rename = "MaskedPaths")]
    pub masked_paths: Option<Vec<String>>,

    /// The list of paths to be set as read-only inside the container (this overrides the default set of paths)
    #[serde(rename = "ReadonlyPaths")]
    pub readonly_paths: Option<Vec<String>>,
}

/// Port binding configuration
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PortBinding {
    /// Host IP address
    #[serde(rename = "HostIp")]
    pub host_ip: Option<String>,

    /// Host port
    #[serde(rename = "HostPort")]
    pub host_port: Option<String>,
}

/// Logging configuration for a container
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LogConfig {
    /// Name of the logging driver
    #[serde(rename = "Type")]
    pub log_type: Option<String>,

    /// Driver-specific configuration options
    #[serde(rename = "Config")]
    pub config: Option<HashMap<String, String>>,
}

/// Restart policy for a container
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RestartPolicy {
    /// Restart policy name
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Maximum number of retries
    #[serde(rename = "MaximumRetryCount")]
    pub maximum_retry_count: Option<i64>,
}

/// A mount for the container
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mount {
    /// Container path
    #[serde(rename = "Target")]
    pub target: Option<String>,

    /// Mount source (e.g. a volume name, a host path)
    #[serde(rename = "Source")]
    pub source: Option<String>,

    /// The mount type
    #[serde(rename = "Type")]
    pub mount_type: Option<String>,

    /// Whether the mount should be read-only
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,

    /// The consistency requirement for the mount
    #[serde(rename = "Consistency")]
    pub consistency: Option<String>,

    /// Optional configuration for the bind type
    #[serde(rename = "BindOptions")]
    pub bind_options: Option<BindOptions>,

    /// Optional configuration for the volume type
    #[serde(rename = "VolumeOptions")]
    pub volume_options: Option<VolumeOptions>,

    /// Optional configuration for the tmpfs type
    #[serde(rename = "TmpfsOptions")]
    pub tmpfs_options: Option<TmpfsOptions>,
}

/// Bind mount options
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BindOptions {
    /// A propagation mode
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,

    /// Disable recursive bind mount
    #[serde(rename = "NonRecursive")]
    pub non_recursive: Option<bool>,

    /// Create mount point on host if missing
    #[serde(rename = "CreateMountpoint")]
    pub create_mountpoint: Option<bool>,

    /// Make the mount non-recursively read-only
    #[serde(rename = "ReadOnlyNonRecursive")]
    pub read_only_non_recursive: Option<bool>,

    /// Make the mount recursively read-only
    #[serde(rename = "ReadOnlyForceRecursive")]
    pub read_only_force_recursive: Option<bool>,
}

/// Volume mount options
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VolumeOptions {
    /// Populate volume with data from the target
    #[serde(rename = "NoCopy")]
    pub no_copy: Option<bool>,

    /// User-defined key/value metadata
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Map of driver specific options
    #[serde(rename = "DriverConfig")]
    pub driver_config: Option<DriverConfig>,

    /// Source path inside the volume
    #[serde(rename = "Subpath")]
    pub subpath: Option<String>,
}

/// Driver configuration
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DriverConfig {
    /// Name of the driver
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Driver options
    #[serde(rename = "Options")]
    pub options: Option<HashMap<String, String>>,
}

/// Tmpfs mount options
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TmpfsOptions {
    /// The size for the tmpfs mount in bytes
    #[serde(rename = "SizeBytes")]
    pub size_bytes: Option<i64>,

    /// The permission mode for the tmpfs mount in an integer
    #[serde(rename = "Mode")]
    pub mode: Option<i64>,

    /// Options to be passed to the tmpfs mount
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,
}

/// Throttle device configuration
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThrottleDevice {
    /// Device path
    #[serde(rename = "Path")]
    pub path: Option<String>,

    /// Rate
    #[serde(rename = "Rate")]
    pub rate: Option<i64>,
}

/// Device mapping between the host and container
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeviceMapping {
    /// Path on the host
    #[serde(rename = "PathOnHost")]
    pub path_on_host: Option<String>,

    /// Path in the container
    #[serde(rename = "PathInContainer")]
    pub path_in_container: Option<String>,

    /// Cgroup permissions
    #[serde(rename = "CgroupPermissions")]
    pub cgroup_permissions: Option<String>,
}

/// A request for devices to be sent to device drivers
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeviceRequest {
    /// Device driver name
    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    /// Number of devices to request
    #[serde(rename = "Count")]
    pub count: Option<i64>,

    /// List of device IDs
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Option<Vec<String>>,

    /// A list of capabilities; an OR list of AND lists of capabilities
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<Vec<Vec<String>>>,

    /// Driver-specific options
    #[serde(rename = "Options")]
    pub options: Option<HashMap<String, String>>,
}

/// Resource limits for a container process
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Ulimit {
    /// Name of the ulimit
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Soft limit
    #[serde(rename = "Soft")]
    pub soft: Option<i64>,

    /// Hard limit
    #[serde(rename = "Hard")]
    pub hard: Option<i64>,
}

/// Information about the container's graph driver
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DriverData {
    /// Name of the storage driver
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Low-level storage metadata
    #[serde(rename = "Data")]
    pub data: Option<HashMap<String, String>>,
}

/// MountPoint represents a mount point configuration inside the container
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MountPoint {
    /// The mount type
    #[serde(rename = "Type")]
    pub mount_type: Option<String>,

    /// Name reference to the underlying data defined by Source
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Source location of the mount
    #[serde(rename = "Source")]
    pub source: Option<String>,

    /// Destination is the path relative to the container root where the Source is mounted
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

    /// Driver is the volume driver used to create the volume
    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    /// Mode is a comma separated list of options supplied by the user when creating the bind/volume mount
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

    /// Whether the mount is mounted writable (read-write)
    #[serde(rename = "RW")]
    pub rw: Option<bool>,

    /// Propagation describes how mounts are propagated from the host into the mount point
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,
}

/// Configuration for a container that is portable between hosts
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContainerConfig {
    /// The hostname to use for the container
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,

    /// The domain name to use for the container
    #[serde(rename = "Domainname")]
    pub domainname: Option<String>,

    /// The user that commands are run as inside the container
    #[serde(rename = "User")]
    pub user: Option<String>,

    /// Whether to attach to stdin
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: Option<bool>,

    /// Whether to attach to stdout
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: Option<bool>,

    /// Whether to attach to stderr
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: Option<bool>,

    /// An object mapping ports to an empty object
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<HashMap<String, Value>>,

    /// Attach standard streams to a TTY
    #[serde(rename = "Tty")]
    pub tty: Option<bool>,

    /// Open stdin
    #[serde(rename = "OpenStdin")]
    pub open_stdin: Option<bool>,

    /// Close stdin after one attached client disconnects
    #[serde(rename = "StdinOnce")]
    pub stdin_once: Option<bool>,

    /// A list of environment variables to set inside the container
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,

    /// Command to run specified as a string or an array of strings
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,

    /// Health check configuration
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<HealthConfig>,

    /// Command is already escaped (Windows only)
    #[serde(rename = "ArgsEscaped")]
    pub args_escaped: Option<bool>,

    /// The name of the image to use when creating the container
    #[serde(rename = "Image")]
    pub image: Option<String>,

    /// An object mapping mount point paths inside the container to empty objects
    #[serde(rename = "Volumes")]
    pub volumes: Option<HashMap<String, Value>>,

    /// The working directory for commands to run in
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,

    /// The entry point for the container as a string or an array of strings
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,

    /// Disable networking for the container
    #[serde(rename = "NetworkDisabled")]
    pub network_disabled: Option<bool>,

    /// MAC address of the container
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

    /// ONBUILD metadata that were defined in the image's Dockerfile
    #[serde(rename = "OnBuild")]
    pub on_build: Option<Vec<String>>,

    /// User-defined key/value metadata
    #[serde(rename = "Labels")]
    pub labels: Option<HashMap<String, String>>,

    /// Signal to stop a container as a string or unsigned integer
    #[serde(rename = "StopSignal")]
    pub stop_signal: Option<String>,

    /// Timeout to stop a container in seconds
    #[serde(rename = "StopTimeout")]
    pub stop_timeout: Option<i64>,

    /// Shell for when RUN, CMD, and ENTRYPOINT uses a shell
    #[serde(rename = "Shell")]
    pub shell: Option<Vec<String>>,
}

/// Health check configuration
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HealthConfig {
    /// The test to perform
    #[serde(rename = "Test")]
    pub test: Option<Vec<String>>,

    /// The time to wait between checks in nanoseconds
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,

    /// The time to wait before considering the check to have hung in nanoseconds
    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,

    /// The number of consecutive failures needed to consider a container as unhealthy
    #[serde(rename = "Retries")]
    pub retries: Option<i64>,

    /// Start period for the container to initialize before starting health-retries countdown in nanoseconds
    #[serde(rename = "StartPeriod")]
    pub start_period: Option<i64>,

    /// The time to wait between checks in nanoseconds during the start period
    #[serde(rename = "StartInterval")]
    pub start_interval: Option<i64>,
}

/// NetworkSettings exposes the network settings in the API
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NetworkSettings {
    /// Name of the default bridge interface when dockerd's --bridge flag is set
    #[serde(rename = "Bridge")]
    pub bridge: Option<String>,

    /// SandboxID uniquely represents a container's network stack
    #[serde(rename = "SandboxID")]
    pub sandbox_id: Option<String>,

    /// Indicates if hairpin NAT should be enabled on the virtual interface
    #[serde(rename = "HairpinMode")]
    pub hairpin_mode: Option<bool>,

    /// IPv6 unicast address using the link-local prefix
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6_address: Option<String>,

    /// Prefix length of the IPv6 unicast address
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6_prefix_len: Option<i64>,

    /// Port mapping
    #[serde(rename = "Ports")]
    pub ports: Option<HashMap<String, Option<Vec<PortBinding>>>>,

    /// SandboxKey is the full path of the netns handle
    #[serde(rename = "SandboxKey")]
    pub sandbox_key: Option<String>,

    /// Secondary IP addresses
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<Address>>,

    /// Secondary IPv6 addresses
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_ipv6_addresses: Option<Vec<Address>>,

    /// EndpointID uniquely represents a service endpoint in a Sandbox
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,

    /// Gateway address for the default "bridge" network
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,

    /// Global IPv6 address for the default "bridge" network
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: Option<String>,

    /// Mask length of the global IPv6 address
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: Option<i64>,

    /// IPv4 address for the default "bridge" network
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,

    /// Mask length of the IPv4 address
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,

    /// IPv6 gateway address for this network
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: Option<String>,

    /// MAC address for the container on the default "bridge" network
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

    /// Information about all networks that the container is connected to
    #[serde(rename = "Networks")]
    pub networks: Option<HashMap<String, EndpointSettings>>,
}

/// Address represents an IPv4 or IPv6 IP address
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Address {
    /// IP address
    #[serde(rename = "Addr")]
    pub addr: Option<String>,

    /// Mask length of the IP address
    #[serde(rename = "PrefixLen")]
    pub prefix_len: Option<i64>,
}

/// EndpointSettings stores the network endpoint details
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EndpointSettings {
    /// Unique ID of the network
    #[serde(rename = "NetworkID")]
    pub network_id: Option<String>,

    /// Unique ID of the service endpoint in a Sandbox
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,

    /// Gateway address for this network
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,

    /// IPv4 address for this network
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,

    /// Mask length of the IPv4 address
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,

    /// IPv6 gateway address for this network
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: Option<String>,

    /// Global IPv6 address for this network
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: Option<String>,

    /// Mask length of the global IPv6 address
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: Option<i64>,

    /// MAC address for the container on this network
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

    /// List of container aliases for this network
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,

    /// List of network driver options
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Option<HashMap<String, String>>,

    /// List of links to other containers
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,

    /// IPAM configuration for this endpoint
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: Option<EndpointIPAMConfig>,

    /// List of DNS names assigned to this endpoint
    #[serde(rename = "DNSNames")]
    pub dns_names: Option<Vec<String>>,
}

/// IPAM configuration for an endpoint
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EndpointIPAMConfig {
    /// IPv4 address
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: Option<String>,

    /// IPv6 address
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: Option<String>,

    /// List of link-local IP addresses
    #[serde(rename = "LinkLocalIPs")]
    pub link_local_ips: Option<Vec<String>>,
}
