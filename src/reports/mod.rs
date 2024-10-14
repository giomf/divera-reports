pub mod absent;
pub mod roster;
pub mod station;

use std::{collections::HashMap, fs::File, path::Path};

use anyhow::{anyhow, Context, Result};
use rust_xlsxwriter::{Format, TableColumn, Worksheet};
use rustydav::client::Client;
use serde_json::Value;
use tempfile::tempdir;

use crate::{config::WebDav, divera::schema::response};

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
    fn upload(self, file_name: &str, config: WebDav) -> Result<()>
    where
        Self: Sized,
    {
        let temp_dir = tempdir().context("Failed to create temp dir")?;
        let file_path = temp_dir.path().join(file_name);
        self.write_xlsx(&file_path)
            .context("Failed to write reports to xlsx")?;

        let webdav_client = Client::init(&config.username, &config.password);
        let path = config.root_directory + "/" + file_name;
        let file = File::open(file_path)?;
        webdav_client
            .put(file, &path)
            .context("Failed to upload reports")?;
        Ok(())
    }
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
