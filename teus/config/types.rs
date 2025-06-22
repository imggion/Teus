use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

/// Represents the deployment environment for the Teus application.
/// 
/// This enum is used to configure environment-specific behavior throughout
/// the application, such as logging levels, database connections, and
/// security settings.
/// 
/// # Examples
/// 
/// ```rust
/// use teus::config::types::Environment;
/// use std::str::FromStr;
/// 
/// let env = Environment::from_str("dev").unwrap();
/// assert_eq!(env.as_str(), "dev");
/// ```
/// 
/// # Serialization
/// 
/// This enum can be serialized and deserialized with serde, making it
/// suitable for use in configuration files (TOML, JSON, etc.).
#[derive(Debug, Clone, Serialize)]
pub enum Environment {
    /// Development environment - typically used for local development
    /// with debug logging and relaxed security settings.
    Development,
    /// Test environment - used for automated testing and QA.
    Test,
    /// Production environment - live deployment with optimized
    /// performance and strict security settings.
    Production,
}

impl<'de> Deserialize<'de> for Environment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Environment::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Environment {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "dev",
            Environment::Test => "test",
            Environment::Production => "prod",
        }
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Environment::Development),
            "test" => Ok(Environment::Test),
            "prod" => Ok(Environment::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

/// Main configuration structure for the Teus application.
/// 
/// This structure holds all the configuration settings required to run
/// the Teus monitoring and management system. It is typically loaded
/// from a TOML configuration file at application startup.
/// 
/// # Structure
/// 
/// The configuration is organized into three main sections:
/// - `server`: Web server and API configuration
/// - `database`: Database connection settings
/// - `monitor`: System monitoring parameters
/// 
/// # Examples
/// 
/// Loading configuration from a TOML file:
/// 
/// ```rust
/// use teus::config::types::Config;
/// 
/// let toml_content = r#"
/// [server]
/// host = "0.0.0.0"
/// port = 8080
/// secret = "your-secret-key"
/// environment = "prod"
/// 
/// [database]
/// path = "./teus.db"
/// 
/// [monitor]
/// interval_secs = 30
/// "#;
/// 
/// let config: Config = toml::from_str(toml_content).unwrap();
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Web server configuration settings
    pub server: ServerConfig,
    /// Database configuration settings
    pub database: DatabaseConfig,
    /// System monitoring configuration settings
    pub monitor: MonitorConfig,
}

/// Configuration for the Teus web server and API.
/// 
/// This structure contains all the settings needed to configure the
/// HTTP server that serves the Teus web interface and API endpoints.
/// 
/// # Security Considerations
/// 
/// The `secret` field is used for JWT token signing and session management.
/// It should be a cryptographically secure random string and must be kept
/// confidential in production environments.
/// 
/// # Examples
/// 
/// ```rust
/// use teus::config::types::{ServerConfig, Environment};
/// 
/// let server_config = ServerConfig {
///     host: "127.0.0.1".to_string(),
///     port: 3000,
///     secret: "secure-random-secret-key".to_string(),
///     environment: Environment::Development,
/// };
/// ```
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// The IP address or hostname the server should bind to.
    /// 
    /// Common values:
    /// - "127.0.0.1" or "localhost" for local-only access
    /// - "0.0.0.0" to bind to all available interfaces
    pub host: String,
    
    /// The TCP port number the server should listen on.
    /// 
    /// Must be a valid port number (1-65535). Common values:
    /// - 8080 for development
    /// - 80 for HTTP in production
    /// - 443 for HTTPS in production
    pub port: u16,
    
    /// Secret key used for JWT token signing and other cryptographic operations.
    /// 
    /// This should be a long, random string. In production, this should be
    /// loaded from environment variables or a secure configuration management
    /// system rather than stored in plain text.
    pub secret: String,
    
    /// The deployment environment this server is running in.
    /// 
    /// Used to configure environment-specific behavior such as logging
    /// levels and security settings. Currently not fully implemented.
    pub environment: Environment,
}

/// Configuration for the SQLite database used by Teus.
/// 
/// Teus uses SQLite as its embedded database for storing system monitoring
/// data, user accounts, and application state. This configuration specifies
/// where the database file should be located.
/// 
/// # File Path Considerations
/// 
/// The path can be:
/// - Relative to the application's working directory
/// - An absolute path
/// - ":memory:" for an in-memory database (testing only)
/// 
/// # Examples
/// 
/// ```rust
/// use teus::config::types::DatabaseConfig;
/// 
/// // Relative path
/// let db_config = DatabaseConfig {
///     path: "./data/teus.db".to_string(),
/// };
/// 
/// // Absolute path
/// let db_config = DatabaseConfig {
///     path: "/var/lib/teus/teus.db".to_string(),
/// };
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Path to the SQLite database file.
    /// 
    /// The directory containing this file must exist and be writable
    /// by the Teus process. If the file doesn't exist, it will be
    /// created automatically on first startup.
    pub path: String,
}

/// Configuration for the system monitoring component.
/// 
/// This structure controls how frequently Teus collects and stores
/// system metrics such as CPU usage, memory consumption, and disk space.
/// 
/// # Performance Considerations
/// 
/// Lower intervals provide more granular data but increase:
/// - CPU overhead from frequent metric collection
/// - Database storage requirements
/// - Memory usage
/// 
/// Higher intervals reduce overhead but may miss short-term spikes
/// in resource usage.
/// 
/// # Recommended Values
/// 
/// - Development/Testing: 5-10 seconds
/// - Production monitoring: 30-60 seconds
/// - Long-term trending: 300+ seconds
/// 
/// # Examples
/// 
/// ```rust
/// use teus::config::types::MonitorConfig;
/// 
/// // High-frequency monitoring
/// let monitor_config = MonitorConfig {
///     interval_secs: 10,
/// };
/// 
/// // Standard production monitoring
/// let monitor_config = MonitorConfig {
///     interval_secs: 60,
/// };
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct MonitorConfig {
    /// Interval between system metric collection cycles, in seconds.
    /// 
    /// This value determines how often Teus will:
    /// - Collect CPU, memory, and disk usage statistics
    /// - Store the collected data in the database
    /// - Update real-time monitoring displays
    /// 
    /// Must be greater than 0. Values less than 5 seconds are not
    /// recommended for production use due to performance overhead.
    pub interval_secs: u64,
}

/// Response structure for the first-visit check API endpoint.
/// 
/// This structure is returned by the API to indicate whether this is
/// the first time the Teus application is being accessed. It's used
/// by the frontend to determine whether to show initial setup screens
/// or proceed to the normal interface.
/// 
/// # API Usage
/// 
/// This structure is typically returned as JSON from the `/api/first-visit`
/// endpoint and is used to drive the initial user experience flow.
/// 
/// # Examples
/// 
/// JSON representation:
/// ```json
/// {
///   "first_visit": true
/// }
/// ```
/// 
/// Rust usage:
/// ```rust
/// use teus::config::types::IsFirstVisitResponse;
/// 
/// let response = IsFirstVisitResponse {
///     first_visit: false,
/// };
/// 
/// let json = serde_json::to_string(&response).unwrap();
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IsFirstVisitResponse {
    /// Indicates whether this is the first visit to the application.
    /// 
    /// - `true`: This is the first time the application is being accessed,
    ///   and initial setup may be required
    /// - `false`: The application has been accessed before and is already
    ///   configured
    pub first_visit: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_as_str() {
        assert_eq!(Environment::Development.as_str(), "dev");
        assert_eq!(Environment::Test.as_str(), "test");
        assert_eq!(Environment::Production.as_str(), "prod");
    }

    #[test]
    fn test_environment_from_str() {
        assert!(matches!(Environment::from_str("dev"), Ok(Environment::Development)));
        assert!(matches!(Environment::from_str("test"), Ok(Environment::Test)));
        assert!(matches!(Environment::from_str("prod"), Ok(Environment::Production)));
        
        // Test error case
        let result = Environment::from_str("invalid");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Unknown environment: invalid");
    }

    #[test]
    fn test_environment_case_sensitivity() {
        // Should fail for different cases
        assert!(Environment::from_str("DEV").is_err());
        assert!(Environment::from_str("Test").is_err());
        assert!(Environment::from_str("PROD").is_err());
    }

    #[test]
    fn test_environment_debug_format() {
        assert_eq!(format!("{:?}", Environment::Development), "Development");
        assert_eq!(format!("{:?}", Environment::Test), "Test");
        assert_eq!(format!("{:?}", Environment::Production), "Production");
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            [server]
            host = "localhost"
            port = 8080
            secret = "secret_key"
            environment = "dev"

            [database]
            path = "./test.db"

            [monitor]
            interval_secs = 60
        "#;

        let config: Result<Config, _> = toml::from_str(toml_str);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.server.host, "localhost");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.secret, "secret_key");
        assert!(matches!(config.server.environment, Environment::Development));
        assert_eq!(config.database.path, "./test.db");
        assert_eq!(config.monitor.interval_secs, 60);
    }

    #[test]
    fn test_server_config_clone() {
        let server_config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 3000,
            secret: "test_secret".to_string(),
            environment: Environment::Test,
        };

        let cloned = server_config.clone();
        assert_eq!(server_config.host, cloned.host);
        assert_eq!(server_config.port, cloned.port);
        assert_eq!(server_config.secret, cloned.secret);
    }

    #[test]
    fn test_is_first_visit_response_serialization() {
        let response = IsFirstVisitResponse { first_visit: true };
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("\"first_visit\":true"));

        let response = IsFirstVisitResponse { first_visit: false };
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("\"first_visit\":false"));
    }

    #[test]
    fn test_is_first_visit_response_deserialization() {
        let json_str = r#"{"first_visit": true}"#;
        let response: IsFirstVisitResponse = serde_json::from_str(json_str).unwrap();
        assert!(response.first_visit);

        let json_str = r#"{"first_visit": false}"#;
        let response: IsFirstVisitResponse = serde_json::from_str(json_str).unwrap();
        assert!(!response.first_visit);
    }

    #[test]
    fn test_config_invalid_environment() {
        let toml_str = r#"
            [server]
            host = "localhost"
            port = 8080
            secret = "secret_key"
            environment = "invalid_env"

            [database]
            path = "./test.db"

            [monitor]
            interval_secs = 60
        "#;

        let config: Result<Config, _> = toml::from_str(toml_str);
        assert!(config.is_err());
    }

    #[test]
    fn test_monitor_config_edge_cases() {
        // Test with very small interval
        let toml_str = r#"
            [server]
            host = "localhost"
            port = 8080
            secret = "secret_key"
            environment = "test"

            [database]
            path = "./test.db"

            [monitor]
            interval_secs = 1
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.monitor.interval_secs, 1);

        // Test with large interval
        let toml_str = r#"
            [server]
            host = "localhost"
            port = 8080
            secret = "secret_key"
            environment = "test"

            [database]
            path = "./test.db"

            [monitor]
            interval_secs = 3600
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.monitor.interval_secs, 3600);
    }
}
