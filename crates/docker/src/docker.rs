use crate::requests::{DockerApi, DockerRequestMethod, TeusRequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(not(target_os = "macos"))]
pub const DOCKER_SOCK: &str = "/var/run/docker.sock";

// For testing purposes, do not forget to replace with your actual Colima or docker path
#[cfg(target_os = "macos")]
pub const DOCKER_SOCK: &str = "/Users/homeerr/.colima/default/docker.sock";

pub type Containers = Vec<Container>;

// A custom error enum for our Docker operations
#[derive(Debug)]
pub enum DockerError {
    Generic(String),           // A catch-all for any unexpected errors
    ContainerNotFound(String), // We can store the container name/ID
    NetworkError(String),      // Store a generic network error message
    DockerDaemonDown,          // A specific state with no extra data
}

#[derive(Debug, Deserialize)]
struct DockerErrorResponse {
    message: String,
}

/* -------------------------
 * Docker Container
 * ----------------------- */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Names")]
    pub names: Vec<String>,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "ImageID")]
    pub image_id: String,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,
    #[serde(rename = "Labels")]
    pub labels: Labels,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "HostConfig")]
    pub host_config: HostConfig,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    #[serde(rename = "PrivatePort")]
    pub private_port: i64,
    #[serde(rename = "PublicPort")]
    pub public_port: Option<i64>,
    #[serde(rename = "Type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Labels {
    #[serde(rename = "com.docker.compose.config-hash")]
    pub com_docker_compose_config_hash: Option<String>,
    #[serde(rename = "com.docker.compose.container-number")]
    pub com_docker_compose_container_number: Option<String>,
    #[serde(rename = "com.docker.compose.depends_on")]
    pub com_docker_compose_depends_on: Option<String>,
    #[serde(rename = "com.docker.compose.image")]
    pub com_docker_compose_image: Option<String>,
    #[serde(rename = "com.docker.compose.oneoff")]
    pub com_docker_compose_oneoff: Option<String>,
    #[serde(rename = "com.docker.compose.project")]
    pub com_docker_compose_project: Option<String>,
    #[serde(rename = "com.docker.compose.project.config_files")]
    pub com_docker_compose_project_config_files: Option<String>,
    #[serde(rename = "com.docker.compose.project.working_dir")]
    pub com_docker_compose_project_working_dir: Option<String>,
    #[serde(rename = "com.docker.compose.service")]
    pub com_docker_compose_service: Option<String>,
    #[serde(rename = "com.docker.compose.version")]
    pub com_docker_compose_version: Option<String>,
    #[serde(rename = "io.portainer.agent")]
    pub io_portainer_agent: Option<String>,
    #[serde(rename = "com.docker.desktop.extension.api.version")]
    pub com_docker_desktop_extension_api_version: Option<String>,
    #[serde(rename = "com.docker.desktop.extension.icon")]
    pub com_docker_desktop_extension_icon: Option<String>,
    #[serde(rename = "com.docker.extension.additional-urls")]
    pub com_docker_extension_additional_urls: Option<String>,
    #[serde(rename = "com.docker.extension.detailed-description")]
    pub com_docker_extension_detailed_description: Option<String>,
    #[serde(rename = "com.docker.extension.publisher-url")]
    pub com_docker_extension_publisher_url: Option<String>,
    #[serde(rename = "com.docker.extension.screenshots")]
    pub com_docker_extension_screenshots: Option<String>,
    #[serde(rename = "io.portainer.server")]
    pub io_portainer_server: Option<String>,
    #[serde(rename = "org.opencontainers.image.description")]
    pub org_opencontainers_image_description: Option<String>,
    #[serde(rename = "org.opencontainers.image.title")]
    pub org_opencontainers_image_title: Option<String>,
    #[serde(rename = "org.opencontainers.image.vendor")]
    pub org_opencontainers_image_vendor: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HostConfig {
    #[serde(rename = "NetworkMode")]
    pub network_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    #[serde(rename = "Networks")]
    pub networks: Networks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Networks {
    pub bridge: Bridge,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bridge {
    #[serde(rename = "IPAMConfig")]
    pub ipamconfig: Value,
    #[serde(rename = "Links")]
    pub links: Value,
    #[serde(rename = "Aliases")]
    pub aliases: Value,
    #[serde(rename = "MacAddress")]
    pub mac_address: String,
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Value,
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
    pub dnsnames: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Mode")]
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    #[serde(rename = "Propagation")]
    pub propagation: String,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
}

/* -------------------------
 * Docker Version
 * ----------------------- */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerVersion {
    #[serde(rename = "Platform")]
    pub platform: Platform,
    #[serde(rename = "Components")]
    pub components: Vec<Component>,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "ApiVersion")]
    pub api_version: String,
    #[serde(rename = "MinAPIVersion")]
    pub min_apiversion: String,
    #[serde(rename = "GitCommit")]
    pub git_commit: String,
    #[serde(rename = "GoVersion")]
    pub go_version: String,
    #[serde(rename = "Os")]
    pub os: String,
    #[serde(rename = "Arch")]
    pub arch: String,
    #[serde(rename = "KernelVersion")]
    pub kernel_version: String,
    #[serde(rename = "BuildTime")]
    pub build_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Details")]
    pub details: Details,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "Arch")]
    pub arch: Option<String>,
    #[serde(rename = "BuildTime")]
    pub build_time: Option<String>,
    #[serde(rename = "Experimental")]
    pub experimental: Option<String>,
    #[serde(rename = "GitCommit")]
    pub git_commit: String,
    #[serde(rename = "GoVersion")]
    pub go_version: Option<String>,
    #[serde(rename = "KernelVersion")]
    pub kernel_version: Option<String>,
    #[serde(rename = "MinAPIVersion")]
    pub min_apiversion: Option<String>,
    #[serde(rename = "Os")]
    pub os: Option<String>,
}

/* -------------------------
 * Docker Info
 * ----------------------- */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerInfo {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Containers")]
    pub containers: i64,
    #[serde(rename = "ContainersRunning")]
    pub containers_running: i64,
    #[serde(rename = "ContainersPaused")]
    pub containers_paused: i64,
    #[serde(rename = "ContainersStopped")]
    pub containers_stopped: i64,
    #[serde(rename = "Images")]
    pub images: i64,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "DriverStatus")]
    pub driver_status: Vec<Vec<String>>,
    #[serde(rename = "Plugins")]
    pub plugins: Plugins,
    #[serde(rename = "MemoryLimit")]
    pub memory_limit: bool,
    #[serde(rename = "SwapLimit")]
    pub swap_limit: bool,
    #[serde(rename = "CpuCfsPeriod")]
    pub cpu_cfs_period: bool,
    #[serde(rename = "CpuCfsQuota")]
    pub cpu_cfs_quota: bool,
    #[serde(rename = "CPUShares")]
    pub cpushares: bool,
    #[serde(rename = "CPUSet")]
    pub cpuset: bool,
    #[serde(rename = "PidsLimit")]
    pub pids_limit: bool,
    #[serde(rename = "IPv4Forwarding")]
    pub ipv4forwarding: bool,
    #[serde(rename = "BridgeNfIptables")]
    pub bridge_nf_iptables: bool,
    #[serde(rename = "BridgeNfIp6tables")]
    pub bridge_nf_ip6tables: bool,
    #[serde(rename = "Debug")]
    pub debug: bool,
    #[serde(rename = "NFd")]
    pub nfd: i64,
    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: bool,
    #[serde(rename = "NGoroutines")]
    pub ngoroutines: i64,
    #[serde(rename = "SystemTime")]
    pub system_time: String,
    #[serde(rename = "LoggingDriver")]
    pub logging_driver: String,
    #[serde(rename = "CgroupDriver")]
    pub cgroup_driver: String,
    #[serde(rename = "CgroupVersion")]
    pub cgroup_version: String,
    #[serde(rename = "NEventsListener")]
    pub nevents_listener: i64,
    #[serde(rename = "KernelVersion")]
    pub kernel_version: String,
    #[serde(rename = "OperatingSystem")]
    pub operating_system: String,
    #[serde(rename = "OSVersion")]
    pub osversion: String,
    #[serde(rename = "OSType")]
    pub ostype: String,
    #[serde(rename = "Architecture")]
    pub architecture: String,
    #[serde(rename = "IndexServerAddress")]
    pub index_server_address: String,
    #[serde(rename = "RegistryConfig")]
    pub registry_config: RegistryConfig,
    #[serde(rename = "NCPU")]
    pub ncpu: i64,
    #[serde(rename = "MemTotal")]
    pub mem_total: i64,
    #[serde(rename = "GenericResources")]
    pub generic_resources: Value,
    #[serde(rename = "DockerRootDir")]
    pub docker_root_dir: String,
    #[serde(rename = "HttpProxy")]
    pub http_proxy: String,
    #[serde(rename = "HttpsProxy")]
    pub https_proxy: String,
    #[serde(rename = "NoProxy")]
    pub no_proxy: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Labels")]
    pub labels: Vec<Value>,
    #[serde(rename = "ExperimentalBuild")]
    pub experimental_build: bool,
    #[serde(rename = "ServerVersion")]
    pub server_version: String,
    #[serde(rename = "Runtimes")]
    pub runtimes: Runtimes,
    #[serde(rename = "DefaultRuntime")]
    pub default_runtime: String,
    #[serde(rename = "Swarm")]
    pub swarm: Swarm,
    #[serde(rename = "LiveRestoreEnabled")]
    pub live_restore_enabled: bool,
    #[serde(rename = "Isolation")]
    pub isolation: String,
    #[serde(rename = "InitBinary")]
    pub init_binary: String,
    #[serde(rename = "ContainerdCommit")]
    pub containerd_commit: ContainerdCommit,
    #[serde(rename = "RuncCommit")]
    pub runc_commit: RuncCommit,
    #[serde(rename = "InitCommit")]
    pub init_commit: InitCommit,
    #[serde(rename = "SecurityOptions")]
    pub security_options: Vec<String>,
    #[serde(rename = "CDISpecDirs")]
    pub cdispec_dirs: Vec<Value>,
    #[serde(rename = "Containerd")]
    pub containerd: Containerd,
    #[serde(rename = "Warnings")]
    pub warnings: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugins {
    #[serde(rename = "Volume")]
    pub volume: Vec<String>,
    #[serde(rename = "Network")]
    pub network: Vec<String>,
    #[serde(rename = "Authorization")]
    pub authorization: Value,
    #[serde(rename = "Log")]
    pub log: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryConfig {
    #[serde(rename = "AllowNondistributableArtifactsCIDRs")]
    pub allow_nondistributable_artifacts_cidrs: Value,
    #[serde(rename = "AllowNondistributableArtifactsHostnames")]
    pub allow_nondistributable_artifacts_hostnames: Value,
    #[serde(rename = "InsecureRegistryCIDRs")]
    pub insecure_registry_cidrs: Vec<String>,
    #[serde(rename = "IndexConfigs")]
    pub index_configs: IndexConfigs,
    #[serde(rename = "Mirrors")]
    pub mirrors: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexConfigs {
    #[serde(rename = "docker.io")]
    pub docker_io: DockerIo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerIo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Mirrors")]
    pub mirrors: Vec<Value>,
    #[serde(rename = "Secure")]
    pub secure: bool,
    #[serde(rename = "Official")]
    pub official: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtimes {
    #[serde(rename = "io.containerd.runc.v2")]
    pub io_containerd_runc_v2: IoContainerdRuncV2,
    pub runc: Runc,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoContainerdRuncV2 {
    pub path: String,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "org.opencontainers.runtime-spec.features")]
    pub org_opencontainers_runtime_spec_features: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runc {
    pub path: String,
    pub status: Status2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status2 {
    #[serde(rename = "org.opencontainers.runtime-spec.features")]
    pub org_opencontainers_runtime_spec_features: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Swarm {
    #[serde(rename = "NodeID")]
    pub node_id: String,
    #[serde(rename = "NodeAddr")]
    pub node_addr: String,
    #[serde(rename = "LocalNodeState")]
    pub local_node_state: String,
    #[serde(rename = "ControlAvailable")]
    pub control_available: bool,
    #[serde(rename = "Error")]
    pub error: String,
    #[serde(rename = "RemoteManagers")]
    pub remote_managers: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerdCommit {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Expected")]
    pub expected: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuncCommit {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Expected")]
    pub expected: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitCommit {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Expected")]
    pub expected: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Containerd {
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Namespaces")]
    pub namespaces: Namespaces,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespaces {
    #[serde(rename = "Containers")]
    pub containers: String,
    #[serde(rename = "Plugins")]
    pub plugins: String,
}

/* -------------------------
 * Docker Volumes
 * ----------------------- */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerVolumes {
    #[serde(rename = "Volumes")]
    pub volumes: Vec<Volume>,
    #[serde(rename = "Warnings")]
    pub warnings: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "Driver")]
    pub driver: String,
    #[serde(rename = "Labels")]
    pub labels: Option<VolumeLabels>,
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Options")]
    pub options: Value,
    #[serde(rename = "Scope")]
    pub scope: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeLabels {
    #[serde(rename = "com.docker.compose.project")]
    pub com_docker_compose_project: Option<String>,
    #[serde(rename = "com.docker.compose.version")]
    pub com_docker_compose_version: Option<String>,
    #[serde(rename = "com.docker.compose.volume")]
    pub com_docker_compose_volume: Option<String>,
    #[serde(rename = "com.docker.volume.anonymous")]
    pub com_docker_volume_anonymous: Option<String>,
}

#[derive(Debug)]
pub struct DockerClient {
    pub request_builder: TeusRequestBuilder,
}

impl DockerClient {
    /// Creates a new DockerClient.
    ///
    /// If `socket_path` is `None`, it defaults to the standard Unix socket path
    /// "/var/run/docker.sock".
    pub fn new(socket_path: Option<String>) -> Self {
        // If socket_path is Some(path), use it.
        // If socket_path is None, execute the closure to get the default path.
        let path = socket_path.unwrap_or_else(|| DOCKER_SOCK.to_string());

        DockerClient {
            // We now pass the guaranteed-to-be-valid path to the builder
            request_builder: TeusRequestBuilder::new(path, "localhost".to_string()).unwrap(),
        }
    }

    /// Helper method to parse Docker responses that might contain errors
    fn parse_docker_response<T>(&self, response: &str) -> Result<T, DockerError>
    where
        T: for<'de> Deserialize<'de>,
    {
        // First, try to deserialize as the expected type
        match serde_json::from_str::<T>(response) {
            Ok(data) => Ok(data),
            Err(_) => {
                // If that fails, try to deserialize as a Docker error response
                match serde_json::from_str::<DockerErrorResponse>(response) {
                    Ok(error_response) => Err(DockerError::Generic(error_response.message)),
                    Err(_) => {
                        // If both fail, return the raw response as a generic error
                        Err(DockerError::Generic(format!(
                            "Failed to parse Docker response: {}",
                            response
                        )))
                    }
                }
            }
        }
    }

    pub fn get_containers(&mut self) -> Result<Containers, DockerError> {
        let response = self
            .request_builder
            .make_request(DockerRequestMethod::Get, DockerApi::Containers);
        self.parse_docker_response(&response)
    }

    pub fn get_container_details(
        &mut self,
        container_id: String,
    ) -> Result<Container, DockerError> {
        let response = self.request_builder.make_request(
            DockerRequestMethod::Get,
            DockerApi::ContainerDetails(container_id),
        );
        self.parse_docker_response(&response)
    }

    pub fn get_version(&mut self) -> Result<DockerVersion, DockerError> {
        let response = self
            .request_builder
            .make_request(DockerRequestMethod::Get, DockerApi::Version);
        self.parse_docker_response(&response)
    }

    pub fn get_volumes(&mut self) -> Result<DockerVolumes, DockerError> {
        let response = self
            .request_builder
            .make_request(DockerRequestMethod::Get, DockerApi::Volumes);
        self.parse_docker_response(&response)
    }

    pub fn get_volume_details(&mut self, volume_name: String) -> Result<Volume, DockerError> {
        let response = self.request_builder.make_request(
            DockerRequestMethod::Get,
            DockerApi::VolumeDetails(volume_name),
        );
        self.parse_docker_response(&response)
    }
}

mod tests {
    use super::*;
    use std::env;

    // For MacOS
    // TODO: Try to get the home directory from the Env
    #[cfg(target_os = "macos")]
    fn get_test_socket_path() -> Option<String> {
        // Path for Colima or Docker Desktop on macOS
        let home_dir = env::var("HOME").unwrap();
        Some(format!("{home_dir}/.colima/default/docker.sock")) 
    }

    // This covers Linux, Windows (via WSL), etc.
    #[cfg(not(target_os = "macos"))]
    fn get_test_socket_path() -> Option<String> {
        // The standard path for Linux. `None` might also be an option if
        // DockerClient's `new` method already defaults to this.
        Some("/var/run/docker.sock".to_string())
    }

    #[test]
    fn test_get_containers() {
        // Our test now calls the correct helper function automatically.
        let test_socket = get_test_socket_path();
        let mut client = DockerClient::new(test_socket);
        println!("{:?}", client);

        let containers = client.get_containers().unwrap();
        println!("{:?}", containers);
        assert!(!containers.is_empty());
    }

    #[test]
    fn test_get_version() {
        // Our test now calls the correct helper function automatically.
        let test_socket = get_test_socket_path();
        let mut client = DockerClient::new(test_socket);
        println!("{:?}", client);

        let version = client.get_version().unwrap();
        println!("{:?}", version);
        assert!(!version.version.is_empty());
    }

    #[test]
    fn test_get_volumes() {
        // Our test now calls the correct helper function automatically.
        let test_socket = get_test_socket_path();
        let mut client = DockerClient::new(test_socket);
        println!("{:?}", client);

        let volumes = client.get_volumes().unwrap();
        println!("{:?}", volumes);
        assert!(!volumes.volumes.is_empty());
    }

    #[test]
    fn test_get_volume_details() {
        // Our test now calls the correct helper function automatically.
        let test_socket = get_test_socket_path();
        let mut client = DockerClient::new(test_socket);
        println!("{:?}", client);

        let volume_name =
            "84146ce4581849ab32389b4fa709e47ce80f2a78075f9a32dbb2f6f8b19456de".to_string();
        let volume_details = client.get_volume_details(volume_name).unwrap();
        println!("{:?}", volume_details);
        assert!(!volume_details.name.is_empty());
    }
}
