use crate::{
    container::ContainerInspectResponse,
    requests::{DockerApi, DockerRequestMethod, TeusRequestBuilder},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
/// Docker container information structure.
///
/// This structure represents comprehensive information about a Docker container
/// as returned by the Docker daemon API. It includes container metadata,
/// runtime state, network configuration, and resource bindings.
///
/// # API Mapping
///
/// Maps to the Docker API `/containers/json` endpoint response format.
/// Field names use serde rename attributes to match Docker's JSON field naming.
///
/// # Usage
///
/// Used for:
/// - Container listing and inventory
/// - Container status monitoring
/// - Runtime configuration inspection
/// - Network and storage analysis
///
/// # Examples
///
/// ```rust
/// use teus::docker::Container;
///
/// // Typically populated from Docker API response
/// let container = Container {
///     id: "abc123def456".to_string(),
///     names: vec!["/my-app".to_string()],
///     image: "nginx:latest".to_string(),
///     state: "running".to_string(),
///     // ... other fields
/// };
/// ```
///
/// # JSON Response Format
///
/// ```json
/// {
///   "Id": "abc123def456",
///   "Names": ["/my-app"],
///   "Image": "nginx:latest",
///   "State": "running",
///   "Status": "Up 2 hours"
/// }
/// ```
pub struct Container {
    /// Unique container identifier (full SHA256 hash).
    ///
    /// This is the complete container ID as assigned by Docker.
    /// Used for all container operations and API calls.
    #[serde(rename = "Id")]
    pub id: String,

    /// Container names as assigned by Docker.
    ///
    /// Typically includes the primary name with leading slash
    /// (e.g., "/my-container") and any aliases. Multiple names
    /// are possible when containers are linked.
    #[serde(rename = "Names")]
    pub names: Vec<String>,

    /// Docker image name and tag used to create this container.
    ///
    /// Format: `repository:tag` or `repository@digest`
    /// Examples: "nginx:latest", "ubuntu:22.04", "redis:alpine"
    #[serde(rename = "Image")]
    pub image: String,

    /// Unique identifier of the Docker image.
    ///
    /// SHA256 hash of the image used to create this container.
    /// Used for image management and container-to-image relationships.
    #[serde(rename = "ImageID")]
    pub image_id: String,

    /// Command executed when the container starts.
    ///
    /// The primary process command line that runs inside the container.
    /// This is either the default command from the image or the
    /// command specified when the container was created.
    #[serde(rename = "Command")]
    pub command: String,

    /// Container creation timestamp (Unix timestamp).
    ///
    /// When the container was created (not started). Used for
    /// sorting, filtering, and lifecycle management.
    #[serde(rename = "Created")]
    pub created: i64,

    /// Network port mappings and exposed ports.
    ///
    /// Contains information about ports exposed by the container
    /// and any host port mappings. Essential for network connectivity
    /// and service discovery.
    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,

    /// Container labels as key-value pairs.
    ///
    /// Metadata labels assigned to the container, including
    /// Docker Compose labels, custom application labels, and
    /// orchestration system labels. Uses HashMap for flexible
    /// label handling when some labels may be missing.
    #[serde(rename = "Labels", default)]
    pub labels: HashMap<String, String>,

    /// Current container state.
    ///
    /// Basic container state: "created", "restarting", "running",
    /// "removing", "paused", "exited", or "dead".
    #[serde(rename = "State")]
    pub state: String,

    /// Human-readable container status description.
    ///
    /// Detailed status information including uptime for running
    /// containers or exit information for stopped containers.
    /// Examples: "Up 2 hours", "Exited (0) 5 minutes ago"
    #[serde(rename = "Status")]
    pub status: String,

    /// Host system configuration for the container.
    ///
    /// Contains resource limits, networking mode, and other
    /// host-level configuration that affects container runtime behavior.
    #[serde(rename = "HostConfig")]
    pub host_config: HostConfig,

    /// Network configuration and connectivity information.
    ///
    /// Details about networks the container is connected to,
    /// IP addresses, and network-related settings.
    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,

    /// Volume and bind mount information.
    ///
    /// Contains details about storage volumes, bind mounts, and
    /// tmpfs mounts attached to the container. Critical for
    /// data persistence and sharing.
    #[serde(rename = "Mounts")]
    pub mounts: Vec<Mount>,
}

/// Container port mapping and exposure information.
///
/// This structure represents network port configuration for Docker containers,
/// including both exposed ports within the container and any mappings to
/// host system ports. Essential for understanding container network accessibility.
///
/// # Port Types
///
/// - **Private Port**: Port exposed inside the container
/// - **Public Port**: Port mapped on the host system (if any)
/// - **IP Address**: Host IP address for the port binding
/// - **Protocol**: Network protocol (tcp, udp, sctp)
///
/// # Examples
///
/// ```rust
/// use teus::docker::Port;
///
/// // HTTP port exposed but not mapped to host
/// let exposed_port = Port {
///     ip: None,
///     private_port: 80,
///     public_port: None,
///     type_field: "tcp".to_string(),
/// };
///
/// // HTTPS port mapped to host port 8443
/// let mapped_port = Port {
///     ip: Some("0.0.0.0".to_string()),
///     private_port: 443,
///     public_port: Some(8443),
///     type_field: "tcp".to_string(),
/// };
/// ```
///
/// # JSON Format
///
/// ```json
/// {
///   "IP": "0.0.0.0",
///   "PrivatePort": 80,
///   "PublicPort": 8080,
///   "Type": "tcp"
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Port {
    /// Host IP address where the port is bound.
    ///
    /// - `None`: Port is exposed but not bound to host
    /// - `Some("0.0.0.0")`: Bound to all host interfaces
    /// - `Some("127.0.0.1")`: Bound only to localhost
    /// - `Some("192.168.1.100")`: Bound to specific IP
    #[serde(rename = "IP")]
    pub ip: Option<String>,

    /// Port number inside the container.
    ///
    /// This is the port that the application inside the container
    /// is listening on. Always present for exposed ports.
    #[serde(rename = "PrivatePort")]
    pub private_port: i64,

    /// Port number on the host system.
    ///
    /// - `None`: Port is exposed but not published to host
    /// - `Some(port)`: Port is mapped to this host port number
    ///
    /// When present, external traffic to this host port will be
    /// forwarded to the container's private port.
    #[serde(rename = "PublicPort")]
    pub public_port: Option<i64>,

    /// Network protocol type.
    ///
    /// Common values:
    /// - "tcp": Transmission Control Protocol
    /// - "udp": User Datagram Protocol
    /// - "sctp": Stream Control Transmission Protocol
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
/// Container host system configuration.
///
/// This structure represents the host-level configuration settings
/// that affect how the container interacts with the host system.
/// Currently focused on network configuration but can be extended
/// for other host-level settings.
///
/// # Network Modes
///
/// Common network mode values:
/// - "bridge": Default Docker bridge network
/// - "host": Use host network stack directly
/// - "none": No network access
/// - "container:<name>": Share network with another container
/// - "<network-name>": Connect to a specific Docker network
///
/// # Examples
///
/// ```rust
/// use teus::docker::HostConfig;
///
/// let bridge_config = HostConfig {
///     network_mode: "bridge".to_string(),
/// };
///
/// let host_config = HostConfig {
///     network_mode: "host".to_string(),
/// };
/// ```
pub struct HostConfig {
    /// Network mode configuration for the container.
    ///
    /// Determines how the container connects to networks and
    /// interacts with the host system's network stack. This
    /// setting affects container isolation and connectivity.
    #[serde(rename = "NetworkMode")]
    pub network_mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Container network configuration and connectivity details.
///
/// This structure contains information about the networks that a container
/// is connected to, including IP addresses, network configuration, and
/// connectivity details for each network.
///
/// # Network Information
///
/// Each network connection includes:
/// - IP address assignments
/// - Network aliases and DNS names
/// - MAC address configuration
/// - Driver-specific options
///
/// # Examples
///
/// ```rust
/// use teus::docker::{NetworkSettings, NetworkDetails};
/// use std::collections::HashMap;
///
/// let mut networks = HashMap::new();
/// networks.insert("bridge".to_string(), NetworkDetails {
///     // ... network details
/// });
///
/// let network_settings = NetworkSettings { networks };
/// ```
pub struct NetworkSettings {
    /// Map of network names to detailed network configuration.
    ///
    /// Each entry represents a network that the container is
    /// connected to, with the key being the network name and
    /// the value containing detailed connection information.
    #[serde(rename = "Networks")]
    pub networks: HashMap<String, NetworkDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Detailed network connection information for a container.
///
/// This structure provides comprehensive details about how a container
/// is connected to a specific Docker network, including IP addressing,
/// DNS configuration, and network-specific settings.
///
/// # Network Configuration
///
/// Contains both IPv4 and IPv6 addressing information, gateway settings,
/// DNS names, and network driver configuration. Essential for understanding
/// container network connectivity and troubleshooting network issues.
///
/// # Examples
///
/// ```rust
/// use teus::docker::NetworkDetails;
/// use serde_json::Value;
///
/// let network_details = NetworkDetails {
///     network_id: "bridge123".to_string(),
///     endpoint_id: "endpoint456".to_string(),
///     gateway: "172.17.0.1".to_string(),
///     ipaddress: "172.17.0.2".to_string(),
///     ipprefix_len: 16,
///     mac_address: Some("02:42:ac:11:00:02".to_string()),
///     // ... other fields
/// };
/// ```
pub struct NetworkDetails {
    /// IP Address Management configuration.
    ///
    /// Contains static IP configuration and IPAM-specific settings
    /// for this network connection. Structure varies by network driver.
    #[serde(rename = "IPAMConfig")]
    pub ipamconfig: Value,

    /// Container links configuration.
    ///
    /// Legacy Docker links to other containers on the same network.
    /// Generally replaced by custom networks and service discovery.
    #[serde(rename = "Links")]
    pub links: Value,

    /// Network aliases for this container.
    ///
    /// DNS names by which this container can be reached on this network.
    /// Used for service discovery within Docker networks.
    #[serde(rename = "Aliases")]
    pub aliases: Value,

    /// MAC address assigned to the container's network interface.
    ///
    /// Hardware address used for this network connection. May be
    /// automatically assigned or explicitly configured.
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

    /// Network driver-specific options.
    ///
    /// Configuration options specific to the network driver being used.
    /// Content varies depending on the network driver (bridge, overlay, etc.).
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Value,

    /// Unique identifier of the Docker network.
    ///
    /// Internal network ID used by Docker to identify the network
    /// this container is connected to.
    #[serde(rename = "NetworkID")]
    pub network_id: String,

    /// Unique identifier of the network endpoint.
    ///
    /// Internal endpoint ID representing this container's connection
    /// point to the network.
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,

    /// IPv4 gateway address for this network.
    ///
    /// Default route gateway that the container uses to reach
    /// addresses outside this network.
    #[serde(rename = "Gateway")]
    pub gateway: String,

    /// IPv4 address assigned to the container on this network.
    ///
    /// Primary IP address used for communication on this network.
    /// Used by other containers and external systems to reach this container.
    #[serde(rename = "IPAddress")]
    pub ipaddress: String,

    /// IPv4 subnet prefix length.
    ///
    /// Number of bits in the network portion of the IP address.
    /// Used to determine the network range and subnet mask.
    #[serde(rename = "IPPrefixLen")]
    pub ipprefix_len: i64,

    /// IPv6 gateway address for this network.
    ///
    /// Default IPv6 gateway for traffic leaving this network.
    /// Empty string if IPv6 is not configured.
    #[serde(rename = "IPv6Gateway")]
    pub ipv6gateway: String,

    /// Global IPv6 address assigned to the container.
    ///
    /// Routable IPv6 address if IPv6 networking is enabled.
    /// Empty string if IPv6 is not configured.
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6address: String,

    /// IPv6 subnet prefix length.
    ///
    /// Number of bits in the IPv6 network portion.
    /// Zero if IPv6 is not configured.
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6prefix_len: i64,

    /// DNS names associated with this container on the network.
    ///
    /// Additional DNS names that can be used to resolve to this
    /// container within the network.
    #[serde(rename = "DNSNames")]
    pub dnsnames: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Container mount and volume information.
///
/// This structure represents storage mounts attached to a Docker container,
/// including volume mounts, bind mounts, and tmpfs mounts. Essential for
/// understanding data persistence and storage configuration.
///
/// # Mount Types
///
/// - **bind**: Bind mount from host filesystem
/// - **volume**: Docker-managed volume
/// - **tmpfs**: Temporary filesystem in memory
///
/// # Access Modes
///
/// - **rw**: Read-write access (default)
/// - **ro**: Read-only access
///
/// # Examples
///
/// ```rust
/// use teus::docker::Mount;
///
/// // Bind mount from host directory
/// let bind_mount = Mount {
///     type_field: "bind".to_string(),
///     source: "/host/data".to_string(),
///     destination: "/app/data".to_string(),
///     mode: "rw".to_string(),
///     rw: true,
///     propagation: "rprivate".to_string(),
/// };
///
/// // Docker volume mount
/// let volume_mount = Mount {
///     type_field: "volume".to_string(),
///     source: "app-data".to_string(),
///     destination: "/app/storage".to_string(),
///     mode: "rw".to_string(),
///     rw: true,
///     propagation: "".to_string(),
/// };
/// ```
pub struct Mount {
    /// Type of mount (bind, volume, tmpfs).
    ///
    /// Determines how the storage is provided:
    /// - "bind": Direct mount from host filesystem
    /// - "volume": Docker-managed named volume
    /// - "tmpfs": Temporary memory-based filesystem
    #[serde(rename = "Type")]
    pub type_field: String,

    /// Source path or volume name.
    ///
    /// For bind mounts: absolute path on host filesystem
    /// For volumes: volume name as managed by Docker
    /// For tmpfs: not applicable (empty string)
    #[serde(rename = "Source")]
    pub source: String,

    /// Mount destination path inside the container.
    ///
    /// Absolute path where the mount appears within the
    /// container's filesystem namespace.
    #[serde(rename = "Destination")]
    pub destination: String,

    /// Access mode string representation.
    ///
    /// Common values:
    /// - "rw": Read-write access
    /// - "ro": Read-only access
    /// - May include additional options
    #[serde(rename = "Mode")]
    pub mode: String,

    /// Read-write access flag.
    ///
    /// - `true`: Mount allows write access
    /// - `false`: Mount is read-only
    #[serde(rename = "RW")]
    pub rw: bool,

    /// Mount propagation setting.
    ///
    /// Controls how mount events propagate between host and container:
    /// - "rprivate": Private (default)
    /// - "shared": Shared propagation
    /// - "slave": Slave propagation
    /// - "rshared": Recursive shared
    /// - "rslave": Recursive slave
    #[serde(rename = "Propagation")]
    pub propagation: String,

    /// Optional name for named volumes.
    ///
    /// For volume mounts, this is the name of the Docker volume.
    /// For bind mounts, this field is typically not used.
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Volume driver name (for volume mounts).
    ///
    /// Specifies the volume driver used for volume mounts.
    /// Common drivers include "local" for local storage.
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
}

/* -------------------------
 * Docker Version
 * ----------------------- */

/// Docker daemon version and build information.
///
/// This structure contains comprehensive version information about the Docker
/// daemon, including API versions, build details, and platform information.
/// Used for compatibility checking and system information reporting.
///
/// # API Compatibility
///
/// The API version fields are crucial for ensuring compatibility between
/// client and server. The `min_apiversion` field indicates the oldest
/// API version supported by this Docker daemon.
///
/// # Examples
///
/// ```rust
/// use teus::docker::DockerVersion;
///
/// // Typically populated from Docker API /version endpoint
/// let version_info = DockerVersion {
///     version: "24.0.5".to_string(),
///     api_version: "1.43".to_string(),
///     min_apiversion: "1.12".to_string(),
///     go_version: "go1.20.6".to_string(),
///     os: "linux".to_string(),
///     arch: "amd64".to_string(),
///     // ... other fields
/// };
/// ```
///
/// # JSON Response Format
///
/// ```json
/// {
///   "Version": "24.0.5",
///   "ApiVersion": "1.43",
///   "MinAPIVersion": "1.12",
///   "GitCommit": "ced0996",
///   "GoVersion": "go1.20.6",
///   "Os": "linux",
///   "Arch": "amd64"
/// }
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerVersion {
    /// Platform information for the Docker daemon.
    ///
    /// Contains details about the platform where Docker is running,
    /// including operating system and architecture information.
    #[serde(rename = "Platform")]
    pub platform: Platform,

    /// List of Docker components and their versions.
    ///
    /// Includes information about Docker Engine, containerd, runc,
    /// and other components that make up the Docker runtime stack.
    #[serde(rename = "Components")]
    pub components: Vec<Component>,

    /// Docker daemon version string.
    ///
    /// The main version identifier for the Docker daemon.
    /// Format typically follows semantic versioning (e.g., "24.0.5").
    #[serde(rename = "Version")]
    pub version: String,

    /// Current Docker API version supported.
    ///
    /// The API version that this Docker daemon currently supports.
    /// Used by clients to determine available features and endpoints.
    #[serde(rename = "ApiVersion")]
    pub api_version: String,

    /// Minimum Docker API version supported.
    ///
    /// The oldest API version that this Docker daemon still supports.
    /// Critical for backward compatibility with older Docker clients.
    #[serde(rename = "MinAPIVersion")]
    pub min_apiversion: String,

    /// Git commit hash of the Docker build.
    ///
    /// Short commit hash identifying the exact source code version
    /// used to build this Docker daemon. Useful for debugging and
    /// exact version identification.
    #[serde(rename = "GitCommit")]
    pub git_commit: String,

    /// Go language version used to build Docker.
    ///
    /// Version of the Go programming language used to compile
    /// the Docker daemon. Important for compatibility and
    /// performance characteristics.
    #[serde(rename = "GoVersion")]
    pub go_version: String,

    /// Operating system where Docker is running.
    ///
    /// The host operating system (e.g., "linux", "windows").
    /// Affects available features and container capabilities.
    #[serde(rename = "Os")]
    pub os: String,

    /// System architecture where Docker is running.
    ///
    /// The CPU architecture (e.g., "amd64", "arm64", "386").
    /// Determines container image compatibility and performance.
    #[serde(rename = "Arch")]
    pub arch: String,

    /// Host kernel version.
    ///
    /// Version of the operating system kernel. Important for
    /// container feature support and security capabilities.
    #[serde(rename = "KernelVersion")]
    pub kernel_version: String,

    /// Docker daemon build timestamp.
    ///
    /// When this version of Docker was compiled and built.
    /// Useful for age assessment and update planning.
    #[serde(rename = "BuildTime")]
    pub build_time: String,
}

/// Docker platform information.
///
/// This structure represents the platform where the Docker daemon
/// is running, providing basic identification of the host environment.
///
/// # Examples
///
/// ```rust
/// use teus::docker::Platform;
///
/// let platform = Platform {
///     name: "Docker Engine - Community".to_string(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    #[serde(rename = "Name")]
    pub name: String,
}

/// Docker component version information.
///
/// This structure represents version and build information for individual
/// components that make up the Docker runtime stack, such as the Docker
/// engine, containerd, and runc.
///
/// # Component Types
///
/// Common components include:
/// - "Engine": Docker daemon itself
/// - "containerd": Container runtime
/// - "runc": OCI runtime
/// - "docker-init": Init process for containers
///
/// # Examples
///
/// ```rust
/// use teus::docker::Component;
///
/// let engine_component = Component {
///     name: "Engine".to_string(),
///     version: "24.0.5".to_string(),
///     details: Details {
///         git_commit: "ced0996".to_string(),
///         // ... other details
///     },
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    /// Name of the Docker component.
    ///
    /// Identifies which part of the Docker stack this component represents.
    /// Examples: "Engine", "containerd", "runc", "docker-init"
    #[serde(rename = "Name")]
    pub name: String,

    /// Version string for this component.
    ///
    /// The specific version of this component in the Docker installation.
    /// May follow different versioning schemes depending on the component.
    #[serde(rename = "Version")]
    pub version: String,

    /// Detailed build and version information for this component.
    ///
    /// Contains additional metadata about the component build,
    /// including commit hashes, build times, and platform details.
    #[serde(rename = "Details")]
    pub details: Details,
}

/// Detailed build information for Docker components.
///
/// This structure contains comprehensive build and version metadata
/// for individual Docker components. Not all fields are present for
/// every component, hence the use of `Option` types.
///
/// # Build Information
///
/// Includes git commit information, build timestamps, Go version used,
/// and platform-specific details that help identify the exact build
/// of each component.
///
/// # Examples
///
/// ```rust
/// use teus::docker::Details;
///
/// let details = Details {
///     git_commit: "de40ad0".to_string(),
///     api_version: Some("1.43".to_string()),
///     go_version: Some("go1.20.6".to_string()),
///     build_time: Some("2023-07-06T19:33:28.000000000+00:00".to_string()),
///     // ... other optional fields
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    /// API version supported by this component (if applicable).
    ///
    /// For components that expose APIs, this indicates the
    /// API version they support. Not all components have APIs.
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<String>,

    /// Architecture this component was built for.
    ///
    /// CPU architecture (e.g., "amd64", "arm64") that this
    /// component binary targets.
    #[serde(rename = "Arch")]
    pub arch: Option<String>,

    /// Timestamp when this component was built.
    ///
    /// ISO 8601 formatted timestamp indicating when this
    /// specific build of the component was created.
    #[serde(rename = "BuildTime")]
    pub build_time: Option<String>,

    /// Whether experimental features are enabled.
    ///
    /// Indicates if this component build includes experimental
    /// or preview features. Values typically "true" or "false".
    #[serde(rename = "Experimental")]
    pub experimental: Option<String>,

    /// Git commit hash for this component build.
    ///
    /// Short commit hash identifying the exact source code
    /// version used to build this component. Always present.
    #[serde(rename = "GitCommit")]
    pub git_commit: String,

    /// Go language version used to build this component.
    ///
    /// Version of Go used for compilation, if the component
    /// is written in Go (most Docker components are).
    #[serde(rename = "GoVersion")]
    pub go_version: Option<String>,

    /// Host kernel version (if relevant to this component).
    ///
    /// Operating system kernel version, included for components
    /// that interact closely with kernel features.
    #[serde(rename = "KernelVersion")]
    pub kernel_version: Option<String>,

    /// Minimum API version supported (if applicable).
    ///
    /// For API-exposing components, the oldest API version
    /// still supported for backward compatibility.
    #[serde(rename = "MinAPIVersion")]
    pub min_apiversion: Option<String>,

    /// Operating system this component was built for.
    ///
    /// Target operating system (e.g., "linux", "windows")
    /// for this component build.
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
    pub request_builder: TeusRequestBuilder, // i think this can be initialized outside of the struct
}

// #[derive(Debug, Deserialize)]
// pub struct ContainersQuery {
//     all: Option<bool>,
// }

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
            request_builder: TeusRequestBuilder::new(path, "localhost".to_string())
                .expect("Are you sure docker is up and running?"),
        }
    }

    /// Helper method to parse Docker responses that might contain errors
    fn parse_docker_response<T>(&self, response: &str) -> Result<T, DockerError>
    where
        T: for<'de> Deserialize<'de>,
    {
        /* T deserialization */
        match serde_json::from_str::<T>(response) {
            Ok(data) => Ok(data),
            Err(_) => {
                match serde_json::from_str::<DockerErrorResponse>(response) {
                    Ok(error_response) => Err(DockerError::Generic(error_response.message)),
                    Err(_) => {
                        /* if both fail, return the raw response as a generic error */
                        Err(DockerError::Generic(format!(
                            "Failed to parse Docker response: {}",
                            response
                        )))
                    }
                }
            }
        }
    }

    pub fn get_containers(&mut self, query: Option<String>) -> Result<Containers, DockerError> {
        let response = self.request_builder.make_request(
            DockerRequestMethod::Get,
            DockerApi::Containers,
            query,
        );
        self.parse_docker_response(&response)
    }

    pub fn get_container_details(
        &mut self,
        container_id: String,
    ) -> Result<ContainerInspectResponse, DockerError> {
        let response = self.request_builder.make_request(
            DockerRequestMethod::Get,
            DockerApi::ContainerDetails(container_id),
            None,
        );
        self.parse_docker_response(&response)
    }

    pub fn get_version(&mut self) -> Result<DockerVersion, DockerError> {
        let response =
            self.request_builder
                .make_request(DockerRequestMethod::Get, DockerApi::Version, None);
        self.parse_docker_response(&response)
    }

    pub fn get_volumes(&mut self) -> Result<DockerVolumes, DockerError> {
        let response =
            self.request_builder
                .make_request(DockerRequestMethod::Get, DockerApi::Volumes, None);
        self.parse_docker_response(&response)
    }

    pub fn get_volume_details(&mut self, volume_name: String) -> Result<Volume, DockerError> {
        let response = self.request_builder.make_request(
            DockerRequestMethod::Get,
            DockerApi::VolumeDetails(volume_name),
            None,
        );
        self.parse_docker_response(&response)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    use std::env;

    // For MacOS
    // TODO: Try to get the home directory from the Env
    #[allow(dead_code)]
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

        let containers = client.get_containers(None).unwrap();
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
    /* this test is not good for everyone */
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
