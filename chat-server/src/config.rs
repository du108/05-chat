use anyhow::{self, Result};
use serde::{Deserialize, Serialize};
// use serde_yaml;
use std::fs::File;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

impl AppConfig {
    pub fn load_config() -> Result<Self, anyhow::Error> {
        // let config = AppConfig::new();
        // Load configuration from file or environment variables here.
        let file = File::open("app.yaml")?;
        let config = serde_yaml::from_reader(file)?;

        Ok(config)
    }
}
