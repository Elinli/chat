use std::fs::File;

use anyhow::{bail, Result};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from user/etc/config/app.yaml or ./app.yaml or fron env chat_config
        let ret = match (
            File::open("app.yml"),
            File::open("/etc/config/app.yml"),
            std::env::var("CHAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),

            (_, Ok(reader), _) => serde_yaml::from_reader(reader),

            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}