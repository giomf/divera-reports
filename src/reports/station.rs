use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, bail, Context, Result};
use comfy_table::{ContentArrangement, Table};

use crate::divera::schema::response;

use super::{parse_string, Reports};

const TYPE_ID: &str = "35e2d05a-1368-43b5-8611-4afc319c95da";
const NOTE_ID: &str = "383b1c3c-4470-440a-bf03-27b315778576";

const TYPE_TEXT: &str = "Art";
const NOTE_TEXT: &str = "Mitteilung";

const TYPE_CLARIFICATION_ID: &str = "97d63a1a-f497-4e2c-bfa4-666038553b7a";
const TYPE_DESIGN_ID: &str = "e499b1dd-5977-47d1-a554-bff91f7e3ef0";
const TYPE_IMPROVEMENT_ID: &str = "afcb458f-635b-43c9-afbb-55280f8fd2f1";
const TYPE_PROBLEM_ID: &str = "ff6b3ae9-9378-4f92-bd4f-b1203c48aff3";

const TYPE_CLARIFICATION_TEXT: &str = "Klärungsbedarf";
const TYPE_DESIGN_TEXT: &str = "Ausgestaltung";
const TYPE_IMPROVEMENT_TEXT: &str = "Verbersserung";
const TYPE_PROBLEM_TEXT: &str = "Problem";

const STATION_REPORTS_HEADERS: [&str; 4] = ["ID", "Mitglied", TYPE_TEXT, NOTE_TEXT];

#[derive(Clone, Debug, Default)]
pub struct StationReport {
    pub id: i64,
    pub user: String,
    pub r#type: Type,
    pub note: String,
}

#[derive(Clone, Debug, Default)]
pub enum Type {
    Clarification,
    Design,
    #[default]
    Improvement,
    Problem,
}

impl StationReport {
    pub fn new_from_report(
        report_type: &response::ReportTypesItem,
        report: &response::Report,
        user: &response::Consumer,
    ) -> Result<Self> {
        let mut station_report = StationReport {
            id: report.id,
            user: user.stdformat_name.clone(),
            ..Default::default()
        };

        for (field, field_type) in report.fields.iter().zip(report_type.fields.iter()) {
            match field_type.id.as_str() {
                TYPE_ID => {
                    let options = field_type
                        .options
                        .clone()
                        .ok_or_else(|| anyhow!("Failed to get type options"))?;
                    let id = parse_string(field).context("Failed to get type id")?;
                    station_report.r#type = Type::new(options, &id)?;
                }
                NOTE_ID => {
                    station_report.note = parse_string(field).context("Failed to get note")?;
                }
                _ => bail!("Unknown station report type \"{}\"", field_type.name),
            };
        }

        Ok(station_report)
    }
}

impl Reports for Vec<StationReport> {
    fn new_from_reports(
        report_type: response::ReportTypesItem,
        reports: response::Reports,
        users: HashMap<String, response::Consumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut station_reports: Vec<StationReport> = Vec::default();

        for report in reports.items {
            let user = users
                .get(&report.user_cluster_relation_id.to_string())
                .cloned()
                .unwrap_or_default();
            let station_report = StationReport::new_from_report(&report_type, &report, &user)
                .context("Failed to create station report")?;
            station_reports.push(station_report);
        }

        Ok(station_reports)
    }

    fn print(self) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(STATION_REPORTS_HEADERS);
        for report in self {
            table.add_row(vec![
                report.id.to_string(),
                report.user,
                report.r#type.to_string(),
                report.note,
            ]);
        }

        println!("{table}");
    }

    fn write_xlsx(&self) {
        todo!()
    }
}

impl Type {
    pub fn new(types: Vec<response::ReportTypesItemFieldOption>, id: &str) -> Result<Self> {
        let r#type = types
            .iter()
            .find(|r#type| r#type.id == id)
            .context(format!("Type id \"{}\" can not be found in types", id))?;

        let variant = match r#type.id.as_str() {
            TYPE_CLARIFICATION_ID => Self::Clarification,
            TYPE_DESIGN_ID => Self::Design,
            TYPE_IMPROVEMENT_ID => Self::Improvement,
            TYPE_PROBLEM_ID => Self::Problem,
            _ => bail!("Unknow type variant \"{}\"", r#type.id),
        };

        Ok(variant)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Clarification => f.write_str(TYPE_CLARIFICATION_TEXT),
            Type::Design => f.write_str(TYPE_DESIGN_TEXT),
            Type::Improvement => f.write_str(TYPE_IMPROVEMENT_TEXT),
            Type::Problem => f.write_str(TYPE_PROBLEM_TEXT),
        }
    }
}
