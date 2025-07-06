use teus_types::config::Config;
use std::error::Error;
use std::{fs, path::Path};

#[allow(dead_code)]
type GeneralError = Box<dyn Error>;
#[allow(dead_code)]
type ConfigResult<T> = Result<T, GeneralError>;

pub fn load_config<P: AsRef<Path>>(path: P) -> ConfigResult<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    fn create_test_config_content() -> String {
        r#"
[server]
host = "127.0.0.1"
port = 8080
secret = "test_secret_key"
environment = "test"

[database]
path = "./test.db"

[monitor]
interval_secs = 60
"#
        .to_string()
    }

    #[test]
    fn test_load_valid_config() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let config_content = create_test_config_content();
        fs::write(temp_file.path(), config_content).expect("Failed to write config");

        let result = load_config(temp_file.path());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.secret, "test_secret_key");
        assert_eq!(config.database.path, "./test.db");
        assert_eq!(config.monitor.interval_secs, 60);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_config("nonexistent_file.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_invalid_toml() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let invalid_content = "invalid toml content [[[";
        fs::write(temp_file.path(), invalid_content).expect("Failed to write invalid config");

        let result = load_config(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_incomplete_config() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let incomplete_content = r#"
[server]
host = "127.0.0.1"
# Missing port, secret, environment
"#;
        fs::write(temp_file.path(), incomplete_content).expect("Failed to write incomplete config");

        let result = load_config(temp_file.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_with_different_environment() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let config_content = r#"
[server]
host = "0.0.0.0"
port = 3000
secret = "prod_secret"
environment = "prod"

[database]
path = "/var/lib/teus/teus.db"

[monitor]
interval_secs = 30
"#;
        fs::write(temp_file.path(), config_content).expect("Failed to write config");

        let result = load_config(temp_file.path());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.monitor.interval_secs, 30);
    }
}
