use crate::config::types::Config;
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
