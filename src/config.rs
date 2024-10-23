use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub divera: Divera,
    pub webdav: WebDav,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Divera {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebDav {
    pub username: String,
    pub password: String,
    pub root_directory: String,
}

impl Config {
    pub fn new(
        divera_username: String,
        divera_password: String,
        webdav_username: String,
        webdav_password: String,
        webdav_directory: String,
    ) -> Self {
        Config {
            divera: Divera {
                username: divera_username,
                password: divera_password,
            },
            webdav: WebDav {
                username: webdav_username,
                password: webdav_password,
                root_directory: webdav_directory,
            },
        }
    }

    pub fn read(path: &Path) -> Result<Self> {
        let config = fs::read_to_string(path).context("Failed to read config")?;
        let config = toml::from_str(&config).context("Failed to parse config")?;
        log::debug!("Read config: {:#?}", config);
        Ok(config)
    }

    pub fn write(&self, path: &Path) -> Result<()> {
        let config = toml::to_string(&self).context("Failed to render config")?;
        fs::write(path, config).context("Failed to write config")?;
        Ok(())
    }
}
