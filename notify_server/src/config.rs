use std::fs::File;

use anyhow::{bail, Result};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth:AuthConfig
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub pk: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub db_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        // read from user/etc/config/app.yaml or ./app.yaml or fron env chat_config
        let ret = match (
            File::open("notify.yml"),
            File::open("/etc/config/notify.yml"),
            std::env::var("NOTIFY_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),

            (_, Ok(reader), _) => serde_yaml::from_reader(reader),

            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };
        Ok(ret?)
    }
}