use std::fs;

use serde::{Deserialize, Serialize};
use toml;

pub const CONFIG_PATH: &str = "./config.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub divera: Divera,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Divera {
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(divera_username: String, divera_password: String) -> Self {
        Config {
            divera: Divera {
                username: divera_username,
                password: divera_password,
            },
        }
    }

    pub fn read() -> Self {
        let config = fs::read_to_string(CONFIG_PATH).expect("Unable to read config");
        toml::from_str(&config).expect("Unable to parse config")
    }

    pub fn write(&self) {
        let config = toml::to_string(&self).expect("Unable to render config");
        fs::write(CONFIG_PATH, config).expect("Unable to write config");
    }
}
