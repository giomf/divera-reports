use std::{collections::HashMap, fmt::Display, path::Path};

use super::{parse_date, parse_string, set_table, Reports};
use crate::divera::schema::response;
use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use comfy_table;
use rust_xlsxwriter::{Format, Workbook};

const BEGIN_ID: &str = "10f05309-e584-4470-a0db-ce6bb15ade34";
const END_ID: &str = "a9246571-63fd-4cdf-b6f1-77d93173b362";
const NOTE_ID: &str = "29091ead-0dca-4546-830a-c4143e0886ec";
const REASON_ID: &str = "f75a352a-0b9c-4c7e-bf7a-e67e6048f1f1";
const REASON_ILLNESS_ID: &str = "cddd7081-d6a9-4869-a3f7-f821ab7a4e2f";
const REASON_PROFESSIONALLY_ID: &str = "1ad668bf-5a17-4f5d-b762-5ce9bb20c0d9";
const REASON_TRAINING_ID: &str = "f4913db8-0112-40d8-8efa-dad361d8829b";
const REASON_VACATION_ID: &str = "5c66e8a3-bb3b-455a-aeb9-a4982f774dd8";

const TITLE: &str = "Abwesenheiten";
const BEGIN_TEXT: &str = "Von";
const END_TEXT: &str = "Bis";
const NOTE_TEXT: &str = "Bemerkung";
const REASON_ILLNESS_TEXT: &str = "Krankheit / Dienstunfähigkeit";
const REASON_PROFESSIONALLY_TEXT: &str = "Beruflich";
const REASON_TEXT: &str = "Grund";
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

impl AbsentReport {
    pub fn new_from_report(
        report_type: &response::ReportTypesItem,
        report: &response::Report,
        user: &response::Consumer,
    ) -> Result<Self> {
        let mut absent_report = AbsentReport {
            id: report.id,
            user: user.stdformat_name.clone(),
            ..Default::default()
        };

        for (field, field_type) in report.fields.iter().zip(report_type.fields.iter()) {
            match field_type.id.as_str() {
                BEGIN_ID => {
                    absent_report.begin =
                        parse_date(field).context("Failed to get begin of absent report")?
                }
                END_ID => {
                    absent_report.end =
                        parse_date(field).context("Failed to get end of absent report")?
                }
                REASON_ID => {
                    let id = parse_string(field).context("Failed to get reason id")?;
                    absent_report.reason = Reason::new(&id)?;
                }
                NOTE_ID => {
                    absent_report.note = parse_string(field).context("Failed to get note")?;
                }
                _ => bail!("Unknown absent report type \"{}\"", field_type.name),
            };
        }
        Ok(absent_report)
    }
}
impl Reports for Vec<AbsentReport> {
    fn new_from_reports(
        report_type: &response::ReportTypesItem,
        reports: response::Reports,
        users: &HashMap<String, response::Consumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut absent_reports: Vec<AbsentReport> = Vec::default();

        for report in reports.items {
            let user = users
                .get(&report.user_cluster_relation_id.to_string())
                .cloned()
                .unwrap_or_default();
            let absent_report = AbsentReport::new_from_report(&report_type, &report, &user)
                .context("Failed to create absent report")?;
            absent_reports.push(absent_report);
        }

        Ok(absent_reports)
    }

    fn print(self) {
        let mut table = comfy_table::Table::new();
        table.set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
        table.set_header(ABSENT_REPORTS_HEADERS);
        for report in self {
            let row = vec![
                report.id.to_string(),
                report.user,
                report.reason.to_string(),
                report.begin.to_string(),
                report.end.to_string(),
                report.note,
            ];
            table.add_row(row);
        }

        println!("{table}");
    }

    fn write_xlsx(self, path: &Path) -> Result<()> {
        let mut workbook = Workbook::new();
        workbook.read_only_recommended();

        let worksheet = workbook.add_worksheet().set_name(TITLE)?;
        set_table(worksheet, &ABSENT_REPORTS_HEADERS, self.len())?;

        let date_format = Format::new().set_num_format("dd.mm.yyyy");
        for (index, report) in self.into_iter().enumerate() {
            let row = (index + 1) as u32;
            worksheet.write(row, 0, report.id)?;
            worksheet.write(row, 1, report.user)?;
            worksheet.write(row, 2, report.reason.to_string())?;
            worksheet.write_datetime_with_format(row, 3, report.begin, &date_format)?;
            worksheet.write_datetime_with_format(row, 4, report.end, &date_format)?;
            worksheet.write(row, 5, report.note)?;
        }
        worksheet.autofit();
        workbook.save(path)?;
        Ok(())
    }
}

impl Reason {
    pub fn new(id: &str) -> Result<Self> {
        let variant = match id {
            REASON_TRAINING_ID => Self::Training,
            REASON_PROFESSIONALLY_ID => Self::Professionally,
            REASON_ILLNESS_ID => Self::Illness,
            REASON_VACATION_ID => Self::Vacation,
            _ => bail!("Unknow type variant \"{}\"", id),
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
