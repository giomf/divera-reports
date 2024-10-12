pub mod absent;
pub mod roster;
pub mod station;

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::divera::schema::response;

pub trait Reports {
    fn new_from_reports(
        report_type: response::ReportTypesItem,
        reports: response::Reports,
        users: HashMap<String, response::Consumer>,
    ) -> Result<Self>
    where
        Self: Sized;
    fn print(self);
    fn write_xlsx(&self);
}

impl Default for response::Consumer {
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
