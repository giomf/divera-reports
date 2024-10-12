pub mod absent;
pub mod roster;

use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::divera::schema::response::Consumer;

impl Default for Consumer {
    fn default() -> Self {
        Self {
            firstname: "Unbekannt".to_string(),
            lastname: "Unbekannt".to_string(),
            stdformat_name: "Unknown U.".to_string(),
        }
    }
}

fn parse_string(value: &Value) -> Result<String> {
    Ok(value
        .as_str()
        .ok_or_else(|| anyhow!("Value not a string"))?
        .to_string())
}
