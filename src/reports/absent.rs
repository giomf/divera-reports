use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, bail, Context, Result};
use chrono::{DateTime, NaiveDate};
use comfy_table::{ContentArrangement, Table};
use divera::schema::response::{ReportTypesItem, ReportTypesItemFieldOption, Reports};
use serde_json::Value;

use super::parse_string;
use crate::divera::{self, schema::response::Consumer};

const REASON_ID: &str = "f75a352a-0b9c-4c7e-bf7a-e67e6048f1f1";
const BEGIN_ID: &str = "10f05309-e584-4470-a0db-ce6bb15ade34";
const END_ID: &str = "a9246571-63fd-4cdf-b6f1-77d93173b362";
const NOTE_ID: &str = "29091ead-0dca-4546-830a-c4143e0886ec";

const REASON_TEXT: &str = "Grund";
const BEGIN_TEXT: &str = "Von";
const END_TEXT: &str = "Bis";
const NOTE_TEXT: &str = "Bemerkung";

const REASON_ILLNESS_ID: &str = "cddd7081-d6a9-4869-a3f7-f821ab7a4e2f";
const REASON_PROFESSIONALLY_ID: &str = "1ad668bf-5a17-4f5d-b762-5ce9bb20c0d9";
const REASON_TRAINING_ID: &str = "f4913db8-0112-40d8-8efa-dad361d8829b";
const REASON_VACATION_ID: &str = "5c66e8a3-bb3b-455a-aeb9-a4982f774dd8";

const REASON_ILLNESS_TEXT: &str = "Krankheit / Dienstunfähigkeit";
const REASON_PROFESSIONALLY_TEXT: &str = "Beruflich";
const REASON_TRAINING_TEXT: &str = "Aus- / Fortbildung";
const REASON_VACATION_TEXT: &str = "Urlaub";

const ABSENT_REPORTS_HEADERS: [&str; 6] = [
    "ID",
    "Mitglied",
    REASON_TEXT,
    BEGIN_TEXT,
    END_TEXT,
    NOTE_TEXT,
];

#[derive(Clone, Debug, Default)]
pub struct AbsentReport {
    pub id: i64,
    pub user: String,
    pub begin: NaiveDate,
    pub end: NaiveDate,
    pub reason: Reason,
    pub note: String,
}

#[derive(Clone, Debug, Default)]
pub enum Reason {
    Illness,
    Professionally,
    Training,
    #[default]
    Vacation,
}

impl Reason {
    pub fn new(types: Vec<ReportTypesItemFieldOption>, id: &str) -> Result<Self> {
        let r#type = types
            .iter()
            .find(|r#type| r#type.id == id)
            .context(format!("Type id \"{}\" can not be found in types", id))?;

        let variant = match r#type.id.as_str() {
            REASON_TRAINING_ID => Self::Training,
            REASON_PROFESSIONALLY_ID => Self::Professionally,
            REASON_ILLNESS_ID => Self::Illness,
            REASON_VACATION_ID => Self::Vacation,
            _ => bail!("Unknow type variant \"{}\"", r#type.id),
        };

        Ok(variant)
    }
}

impl Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::Illness => f.write_str(REASON_ILLNESS_TEXT),
            Reason::Professionally => f.write_str(REASON_PROFESSIONALLY_TEXT),
            Reason::Training => f.write_str(REASON_TRAINING_TEXT),
            Reason::Vacation => f.write_str(REASON_VACATION_TEXT),
        }
    }
}

pub fn create_absent_reports(
    report_type: ReportTypesItem,
    reports: Reports,
    users: HashMap<String, Consumer>,
) -> Result<Vec<AbsentReport>> {
    let mut absent_reports: Vec<AbsentReport> = Vec::default();

    for report in reports.items {
        let user = users
            .get(&report.user_cluster_relation_id.to_string())
            .cloned()
            .unwrap_or_default();

        let mut absent_report = AbsentReport {
            id: report.id,
            user: user.stdformat_name.clone(),
            ..Default::default()
        };

        for (field, field_type) in report.fields.iter().zip(report_type.fields.iter()) {
            match field_type.id.as_str() {
                BEGIN_ID => {
                    absent_report.begin =
                        parse_date(field).context("Failed to set begin of absent report")?
                }
                END_ID => {
                    absent_report.end =
                        parse_date(field).context("Failed to set end of absent report")?
                }
                REASON_ID => {
                    let options = field_type
                        .options
                        .clone()
                        .ok_or_else(|| anyhow!("Failed to get reason options"))?;
                    let id = parse_string(field).context("Failed to get reason id")?;
                    absent_report.reason = Reason::new(options, &id)?;
                }
                NOTE_ID => {
                    absent_report.note = parse_string(field).context("Failed to get note")?;
                }
                _ => bail!("Unknown absent report type \"{}\"", field_type.name),
            };
        }
        absent_reports.push(absent_report);
    }

    Ok(absent_reports)
}
pub fn print_absent_reports(reports: Vec<AbsentReport>) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(ABSENT_REPORTS_HEADERS);
    for report in reports {
        table.add_row(vec![
            report.id.to_string(),
            report.user,
            report.reason.to_string(),
            report.begin.to_string(),
            report.end.to_string(),
            report.note,
        ]);
    }

    println!("{table}");
}
fn parse_date(value: &Value) -> Result<NaiveDate> {
    let timestamp: i64 = if value.is_number() {
        value.as_i64().unwrap()
    } else {
        // Since divera has strange coding for their date types
        // (Sometimes 12342134 and sometimes 12342134.0) this hack is needed.
        // First get the string, parse it into a float and then truncate it...
        value
            .as_str()
            .ok_or_else(|| anyhow!("Failed to get string from value"))?
            .parse::<f64>()?
            .trunc() as i64
    };
    let datetime = DateTime::from_timestamp(timestamp as i64, 0)
        .ok_or_else(|| anyhow!("Failed to parse \"{}\" to datetime", timestamp))?
        .naive_utc()
        .date();
    Ok(datetime)
}
