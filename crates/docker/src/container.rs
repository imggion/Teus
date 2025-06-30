use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct ContainerExtended {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created", default)]
    pub created: Option<String>,
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Args")]
    pub args: Vec<String>,
    #[serde(rename = "State", default)]
    pub state: Option<State>, // opt this
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: String,
    #[serde(rename = "HostnamePath")]
    pub hostname_path: String,
    #[serde(rename = "HostsPath")]
    pub hosts_path: String,
    #[serde(rename = "LogPath", default)]
    pub log_path: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RestartCount")]
    pub restart_count: i64,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Platform")]
    pub platform: String,
    #[serde(rename = "ImageManifestDescriptor", default)]
    pub image_manifest_descriptor: Option<ImageManifestDescriptor>, // opt this (inside)
    #[serde(rename = "MountLabel")]
    pub mount_label: String,
    #[serde(rename = "ProcessLabel")]
    pub process_label: String,
    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: String,
    #[serde(rename = "ExecIDs")]
    pub exec_ids: Option<Vec<String>>,
    #[serde(rename = "HostConfig")]
    pub host_config: HostConfig, // opt this (inside)
    #[serde(rename = "GraphDriver")]
    pub graph_driver: GraphDriver,
    #[serde(rename = "SizeRw")]
    pub size_rw: Option<String>,
    #[serde(rename = "SizeRootFs")]
    pub size_root_fs: Option<String>,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount2>,
    #[serde(rename = "Config")]
    pub config: Config2,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Running")]
    pub running: bool,
    #[serde(rename = "Paused")]
    pub paused: bool,
    #[serde(rename = "Restarting")]
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oomkilled: bool,
    #[serde(rename = "Dead")]
    pub dead: bool,
    #[serde(rename = "Pid")]
    pub pid: i64,
    #[serde(rename = "ExitCode")]
    pub exit_code: i64,
    #[serde(rename = "Error")]
    pub error: String,
    #[serde(rename = "StartedAt")]
    pub started_at: String,
    #[serde(rename = "FinishedAt")]
    pub finished_at: String,
    #[serde(rename = "Health", default, skip_serializing_if = "Option::is_none")]
    pub health: Option<Health>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "FailingStreak")]
    pub failing_streak: i64,
    #[serde(rename = "Log")]
    pub log: Vec<Log>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "End")]
    pub end: String,
    #[serde(rename = "ExitCode")]
    pub exit_code: i64,
    #[serde(rename = "Output")]
    pub output: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageManifestDescriptor {
    pub media_type: String,
    pub digest: String,
    pub size: i64,
    pub urls: Vec<String>,
    pub annotations: Annotations,
    pub data: Value,
    pub platform: Platform,
    pub artifact_type: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    #[serde(rename = "com.docker.official-images.bashbrew.arch")]
    pub com_docker_official_images_bashbrew_arch: String,
    #[serde(rename = "org.opencontainers.image.base.digest")]
    pub org_opencontainers_image_base_digest: String,
    #[serde(rename = "org.opencontainers.image.base.name")]
    pub org_opencontainers_image_base_name: String,
    #[serde(rename = "org.opencontainers.image.created")]
    pub org_opencontainers_image_created: String,
    #[serde(rename = "org.opencontainers.image.revision")]
    pub org_opencontainers_image_revision: String,
    #[serde(rename = "org.opencontainers.image.source")]
    pub org_opencontainers_image_source: String,
    #[serde(rename = "org.opencontainers.image.url")]
    pub org_opencontainers_image_url: String,
    #[serde(rename = "org.opencontainers.image.version")]
    pub org_opencontainers_image_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    pub architecture: String,
    pub os: String,
    #[serde(rename = "os.version")]
    pub os_version: String,
    #[serde(rename = "os.features")]
    pub os_features: Vec<String>,
    pub variant: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostConfig {
    #[serde(rename = "CpuShares")]
    pub cpu_shares: i64,
    #[serde(rename = "Memory")]
    pub memory: i64,
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: String,
    #[serde(rename = "BlkioWeight")]
    pub blkio_weight: i64,
    #[serde(rename = "BlkioWeightDevice")]
    pub blkio_weight_device: Vec<BlkioWeightDevice>,
    #[serde(rename = "BlkioDeviceReadBps")]
    pub blkio_device_read_bps: Vec<BlkioDeviceReadBp>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    pub blkio_device_write_bps: Vec<BlkioDeviceWriteBp>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_iops: Vec<BlkioDeviceReadIop>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_iops: Vec<BlkioDeviceWriteIop>,
    #[serde(rename = "CpuPeriod")]
    pub cpu_period: i64,
    #[serde(rename = "CpuQuota")]
    pub cpu_quota: i64,
    #[serde(rename = "CpuRealtimePeriod")]
    pub cpu_realtime_period: i64,
    #[serde(rename = "CpuRealtimeRuntime")]
    pub cpu_realtime_runtime: i64,
    #[serde(rename = "CpusetCpus")]
    pub cpuset_cpus: String,
    #[serde(rename = "CpusetMems")]
    pub cpuset_mems: String,
    #[serde(rename = "Devices")]
    pub devices: Vec<Device>,
    #[serde(rename = "DeviceCgroupRules")]
    pub device_cgroup_rules: Vec<String>,
    #[serde(rename = "DeviceRequests")]
    pub device_requests: Vec<DeviceRequest>,
    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: i64,
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: i64,
    #[serde(rename = "MemorySwap")]
    pub memory_swap: i64,
    #[serde(rename = "MemorySwappiness")]
    pub memory_swappiness: i64,
    #[serde(rename = "NanoCpus")]
    pub nano_cpus: i64,
    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: bool,
    #[serde(rename = "Init")]
    pub init: bool,
    #[serde(rename = "PidsLimit")]
    pub pids_limit: i64,
    #[serde(rename = "Ulimits")]
    pub ulimits: Vec<Ulimit>,
    #[serde(rename = "CpuCount")]
    pub cpu_count: i64,
    #[serde(rename = "CpuPercent")]
    pub cpu_percent: i64,
    #[serde(rename = "IOMaximumIOps")]
    pub iomaximum_iops: i64,
    #[serde(rename = "IOMaximumBandwidth")]
    pub iomaximum_bandwidth: i64,
    #[serde(rename = "Binds")]
    pub binds: Vec<String>,
    #[serde(rename = "ContainerIDFile")]
    pub container_idfile: String,
    #[serde(rename = "LogConfig")]
    pub log_config: LogConfig,
    #[serde(rename = "NetworkMode")]
    pub network_mode: String,
    #[serde(rename = "PortBindings")]
    pub port_bindings: PortBindings,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: RestartPolicy,
    #[serde(rename = "AutoRemove")]
    pub auto_remove: bool,
    #[serde(rename = "VolumeDriver")]
    pub volume_driver: String,
    #[serde(rename = "VolumesFrom")]
    pub volumes_from: Vec<String>,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount>,
    #[serde(rename = "ConsoleSize")]
    pub console_size: Vec<i64>,
    #[serde(rename = "Annotations")]
    pub annotations: Annotations2,
    #[serde(rename = "CapAdd")]
    pub cap_add: Vec<String>,
    #[serde(rename = "CapDrop")]
    pub cap_drop: Vec<String>,
    #[serde(rename = "CgroupnsMode")]
    pub cgroupns_mode: String,
    #[serde(rename = "Dns")]
    pub dns: Vec<String>,
    #[serde(rename = "DnsOptions")]
    pub dns_options: Vec<String>,
    #[serde(rename = "DnsSearch")]
    pub dns_search: Vec<String>,
    #[serde(rename = "ExtraHosts")]
    pub extra_hosts: Vec<String>,
    #[serde(rename = "GroupAdd")]
    pub group_add: Vec<String>,
    #[serde(rename = "IpcMode")]
    pub ipc_mode: String,
    #[serde(rename = "Cgroup")]
    pub cgroup: String,
    #[serde(rename = "Links")]
    pub links: Vec<String>,
    #[serde(rename = "OomScoreAdj")]
    pub oom_score_adj: i64,
    #[serde(rename = "PidMode")]
    pub pid_mode: String,
    #[serde(rename = "Privileged")]
    pub privileged: bool,
    #[serde(rename = "PublishAllPorts")]
    pub publish_all_ports: bool,
    #[serde(rename = "ReadonlyRootfs")]
    pub readonly_rootfs: bool,
    #[serde(rename = "SecurityOpt")]
    pub security_opt: Vec<String>,
    #[serde(rename = "StorageOpt")]
    pub storage_opt: StorageOpt,
    #[serde(rename = "Tmpfs")]
    pub tmpfs: Tmpfs,
    #[serde(rename = "UTSMode")]
    pub utsmode: String,
    #[serde(rename = "UsernsMode")]
    pub userns_mode: String,
    #[serde(rename = "ShmSize")]
    pub shm_size: i64,
    #[serde(rename = "Sysctls")]
    pub sysctls: Sysctls,
    #[serde(rename = "Runtime")]
    pub runtime: String,
    #[serde(rename = "Isolation")]
    pub isolation: String,
    #[serde(rename = "MaskedPaths")]
    pub masked_paths: Vec<String>,
    #[serde(rename = "ReadonlyPaths")]
    pub readonly_paths: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlkioWeightDevice {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Weight")]
    pub weight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlkioDeviceReadBp {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Rate")]
    pub rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlkioDeviceWriteBp {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Rate")]
    pub rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlkioDeviceReadIop {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Rate")]
    pub rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlkioDeviceWriteIop {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Rate")]
    pub rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(rename = "PathOnHost")]
    pub path_on_host: String,
    #[serde(rename = "PathInContainer")]
    pub path_in_container: String,
    #[serde(rename = "CgroupPermissions")]
    pub cgroup_permissions: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRequest {
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Count")]
    pub count: i64,
    #[serde(rename = "DeviceIDs")]
    pub device_ids: Vec<String>,
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<Vec<String>>,
    #[serde(rename = "Options")]
    pub options: Options,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub property1: String,
    pub property2: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ulimit {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Soft")]
    pub soft: i64,
    #[serde(rename = "Hard")]
    pub hard: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Config")]
    pub config: Config,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "max-file")]
    pub max_file: String,
    #[serde(rename = "max-size")]
    pub max_size: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortBindings {
    #[serde(rename = "443/tcp")]
    pub n443_tcp: Vec<n443Tcp>,
    #[serde(rename = "80/tcp")]
    pub n80_tcp: Vec<n80Tcp>,
    #[serde(rename = "80/udp")]
    pub n80_udp: Vec<n80Udp>,
    #[serde(rename = "53/udp")]
    pub n53_udp: Vec<n53Udp>,
    #[serde(rename = "2377/tcp")]
    pub n2377_tcp: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n443Tcp {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n80Tcp {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n80Udp {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n53Udp {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartPolicy {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MaximumRetryCount")]
    pub maximum_retry_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "ReadOnly")]
    pub read_only: bool,
    #[serde(rename = "Consistency")]
    pub consistency: String,
    #[serde(rename = "BindOptions")]
    pub bind_options: BindOptions,
    #[serde(rename = "VolumeOptions")]
    pub volume_options: VolumeOptions,
    #[serde(rename = "ImageOptions")]
    pub image_options: ImageOptions,
    #[serde(rename = "TmpfsOptions")]
    pub tmpfs_options: TmpfsOptions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindOptions {
    #[serde(rename = "Propagation")]
    pub propagation: String,
    #[serde(rename = "NonRecursive")]
    pub non_recursive: bool,
    #[serde(rename = "CreateMountpoint")]
    pub create_mountpoint: bool,
    #[serde(rename = "ReadOnlyNonRecursive")]
    pub read_only_non_recursive: bool,
    #[serde(rename = "ReadOnlyForceRecursive")]
    pub read_only_force_recursive: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeOptions {
    #[serde(rename = "NoCopy")]
    pub no_copy: bool,
    #[serde(rename = "Labels")]
    pub labels: Labels,
    #[serde(rename = "DriverConfig")]
    pub driver_config: DriverConfig,
    #[serde(rename = "Subpath")]
    pub subpath: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    pub property1: String,
    pub property2: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverConfig {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Options")]
    pub options: Options2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options2 {
    pub property1: String,
    pub property2: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageOptions {
    #[serde(rename = "Subpath")]
    pub subpath: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TmpfsOptions {
    #[serde(rename = "SizeBytes")]
    pub size_bytes: i64,
    #[serde(rename = "Mode")]
    pub mode: i64,
    #[serde(rename = "Options")]
    pub options: Vec<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations2 {
    pub property1: String,
    pub property2: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageOpt {
    pub property1: String,
    pub property2: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tmpfs {
    pub property1: String,
    pub property2: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sysctls {
    #[serde(rename = "net.ipv4.ip_forward")]
    pub net_ipv4_ip_forward: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDriver {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Data")]
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "MergedDir")]
    pub merged_dir: String,
    #[serde(rename = "UpperDir")]
    pub upper_dir: String,
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount2 {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    #[serde(rename = "Propagation")]
    pub propagation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config2 {
    #[serde(rename = "Hostname")]
    pub hostname: String,
    #[serde(rename = "Domainname")]
    pub domainname: String,
    #[serde(rename = "User")]
    pub user: String,
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: bool,
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: bool,
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: ExposedPorts,
    #[serde(rename = "Tty")]
    pub tty: bool,
    #[serde(rename = "OpenStdin")]
    pub open_stdin: bool,
    #[serde(rename = "StdinOnce")]
    pub stdin_once: bool,
    #[serde(rename = "Env")]
    pub env: Vec<String>,
    #[serde(rename = "Cmd")]
    pub cmd: Vec<String>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Healthcheck,
    #[serde(rename = "ArgsEscaped")]
    pub args_escaped: bool,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "Volumes")]
    pub volumes: Volumes,
    #[serde(rename = "WorkingDir")]
    pub working_dir: String,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<Value>,
    #[serde(rename = "NetworkDisabled")]
    pub network_disabled: bool,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "OnBuild")]
    pub on_build: Vec<Value>,
    #[serde(rename = "Labels")]
    pub labels: Labels2,
    #[serde(rename = "StopSignal")]
    pub stop_signal: String,
    #[serde(rename = "StopTimeout")]
    pub stop_timeout: i64,
    #[serde(rename = "Shell")]
    pub shell: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExposedPorts {
    #[serde(rename = "80/tcp")]
    pub n80_tcp: n80Tcp2,
    #[serde(rename = "443/tcp")]
    pub n443_tcp: n443Tcp2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n80Tcp2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n443Tcp2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healthcheck {
    #[serde(rename = "Test")]
    pub test: Vec<String>,
    #[serde(rename = "Interval")]
    pub interval: i64,
    #[serde(rename = "Timeout")]
    pub timeout: i64,
    #[serde(rename = "Retries")]
    pub retries: i64,
    #[serde(rename = "StartPeriod")]
    pub start_period: i64,
    #[serde(rename = "StartInterval")]
    pub start_interval: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volumes {
    pub property1: Property1,
    pub property2: Property2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property1 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels2 {
    #[serde(rename = "com.example.some-label")]
    pub com_example_some_label: String,
    #[serde(rename = "com.example.some-other-label")]
    pub com_example_some_other_label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    #[serde(rename = "Bridge")]
    pub bridge: String,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: String,
    #[serde(rename = "HairpinMode")]
    pub hairpin_mode: bool,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6address: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6prefix_len: String,
    #[serde(rename = "Ports")]
    pub ports: Ports,
    #[serde(rename = "SandboxKey")]
    pub sandbox_key: String,
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ipaddresses: Vec<SecondaryIpaddress>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_ipv6addresses: Vec<SecondaryIpv6Address>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6prefix_len: i64,
    #[serde(rename = "IPAddress")]
    pub ipaddress: String,
    #[serde(rename = "IPPrefixLen")]
    pub ipprefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6gateway: String,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "Networks")]
    pub networks: Networks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ports {
    #[serde(rename = "443/tcp")]
    pub n443_tcp: Vec<n443Tcp3>,
    #[serde(rename = "80/tcp")]
    pub n80_tcp: Vec<n80Tcp3>,
    #[serde(rename = "80/udp")]
    pub n80_udp: Vec<n80Udp2>,
    #[serde(rename = "53/udp")]
    pub n53_udp: Vec<n53Udp2>,
    #[serde(rename = "2377/tcp")]
    pub n2377_tcp: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n443Tcp3 {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n80Tcp3 {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n80Udp2 {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct n53Udp2 {
    #[serde(rename = "HostIp")]
    pub host_ip: String,
    #[serde(rename = "HostPort")]
    pub host_port: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryIpaddress {
    #[serde(rename = "Addr")]
    pub addr: String,
    #[serde(rename = "PrefixLen")]
    pub prefix_len: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryIpv6Address {
    #[serde(rename = "Addr")]
    pub addr: String,
    #[serde(rename = "PrefixLen")]
    pub prefix_len: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Networks {
    pub property1: Property12,
    pub property2: Property22,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property12 {
    #[serde(rename = "IPAMConfig")]
    pub ipamconfig: Ipamconfig,
    #[serde(rename = "Links")]
    pub links: Vec<String>,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "Aliases")]
    pub aliases: Vec<String>,
    #[serde(rename = "DriverOpts")]
    pub driver_opts: DriverOpts,
    #[serde(rename = "GwPriority")]
    pub gw_priority: Vec<i64>,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ipaddress: String,
    #[serde(rename = "IPPrefixLen")]
    pub ipprefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6prefix_len: i64,
    #[serde(rename = "DNSNames")]
    pub dnsnames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ipamconfig {
    #[serde(rename = "IPv4Address")]
    pub ipv4address: String,
    #[serde(rename = "IPv6Address")]
    pub ipv6address: String,
    #[serde(rename = "LinkLocalIPs")]
    pub link_local_ips: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverOpts {
    #[serde(rename = "com.example.some-label")]
    pub com_example_some_label: String,
    #[serde(rename = "com.example.some-other-label")]
    pub com_example_some_other_label: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property22 {
    #[serde(rename = "IPAMConfig")]
    pub ipamconfig: Ipamconfig2,
    #[serde(rename = "Links")]
    pub links: Vec<String>,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "Aliases")]
    pub aliases: Vec<String>,
    #[serde(rename = "DriverOpts")]
    pub driver_opts: DriverOpts2,
    #[serde(rename = "GwPriority")]
    pub gw_priority: Vec<i64>,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    #[serde(rename = "Gateway")]
    pub gateway: String,
    #[serde(rename = "IPAddress")]
    pub ipaddress: String,
    #[serde(rename = "IPPrefixLen")]
    pub ipprefix_len: i64,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6prefix_len: i64,
    #[serde(rename = "DNSNames")]
    pub dnsnames: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ipamconfig2 {
    #[serde(rename = "IPv4Address")]
    pub ipv4address: String,
    #[serde(rename = "IPv6Address")]
    pub ipv6address: String,
    #[serde(rename = "LinkLocalIPs")]
    pub link_local_ips: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverOpts2 {
    #[serde(rename = "com.example.some-label")]
    pub com_example_some_label: String,
    #[serde(rename = "com.example.some-other-label")]
    pub com_example_some_other_label: String,
}
