use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize)]
pub enum Environment {
    Development,
    Test,
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

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub monitor: MonitorConfig,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub secret: String,
    pub environment: Environment, // Not used yet
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonitorConfig {
    pub interval_secs: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IsFirstVisitResponse {
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
