use std::path::Path;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("json serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Loads configuration from a YAML file
pub fn load_config_from_yaml(path: impl AsRef<Path>) -> Result<Value, LoadError> {
    let content = std::fs::read_to_string(path)?;
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;
    let json_value = serde_json::to_value(yaml_value)?;
    Ok(json_value)
}
