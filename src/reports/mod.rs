pub mod absent;
pub mod roster;
pub mod station;

use std::{collections::HashMap, path::Path};

use anyhow::{anyhow, Context, Result};
use rust_xlsxwriter::{Format, TableColumn, Worksheet};
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
    fn write_xlsx(self, path: &Path) -> Result<()>;
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

fn set_table(worksheet: &mut Worksheet, headers: &[&str], table_length: usize) -> Result<()> {
    let mut table_headers = Vec::default();
    let format = Format::new().set_bold();

    for header in headers {
        table_headers.push(
            TableColumn::new()
                .set_header(header.to_string())
                .set_header_format(&format),
        );
    }
    let table = rust_xlsxwriter::Table::new().set_columns(&table_headers);
    worksheet
        .add_table(
            0,
            0,
            table_length as u32 - 1,
            headers.len() as u16 - 1,
            &table,
        )
        .context("Failed to create table")?;
    Ok(())
}

fn parse_string(value: &Value) -> Result<String> {
    Ok(value
        .as_str()
        .ok_or_else(|| anyhow!("Value not a string"))?
        .to_string())
}
