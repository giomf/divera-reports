use std::{collections::HashMap, fmt::Display, path::Path};

use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use comfy_table::{self, ContentArrangement, Table};
use rust_xlsxwriter::Workbook;
use serde_json::Value;

use super::{parse_date, parse_string, set_table, Reports};
use crate::divera::schema::response::{self};

const ACTIVITIES_ID: &str = "0fb3a9ca-cf80-47ef-bb60-3a365b1877dc";
const ACTIVITY_COBRA_ID: &str = "c5667814-1820-4a82-9272-3364c136a902";
const ACTIVITY_FIRE_FIGHTING_ID: &str = "e3fdc401-8a3b-4e83-aef8-a2c75abab7a0";
const ACTIVITY_LEADING_ID: &str = "f1a5b65d-eb05-41b6-aac4-7339da09e3e4";
const ACTIVITY_RESCUING_ID: &str = "397ced39-22f6-4996-85a2-ef36ac240c7e";
const ACTIVITY_ROOF_OPENING_ID: &str = "5fbc96e9-4e62-4595-ade3-95cba510f032";
const ACTIVITY_VENTILATION_ID: &str = "9affc34f-b981-4cb2-b187-a0b87ea85157";
const DATE_ID: &str = "a4651554-4f5a-472a-bdbb-a051862a2c9c";
const DOUBLE_BOTTLES_ID: &str = "ddf8e441-d849-44ee-bee1-97bd5d340b1b";
const DURATION_ID: &str = "baa7c1b9-af31-4c25-9d84-ae281188ca7c";
const ISSUES_ID: &str = "3c293ad3-632e-42a9-85bf-9d7fcd0e12ad";
const OPERATION_TYPE_ID: &str = "30b19a39-caf7-40ed-ba08-5edcd9e03698";
const SINGLE_BOTTLES_ID: &str = "5e5de223-101e-422c-bc0d-510a6501073b";
const TYPE_ID: &str = "2ef454dc-336a-4af7-89f3-cd985998360b";
const TYPE_OPERATION_ID: &str = "6481edc4-4754-4b28-a9b6-220154740fb7";
const TYPE_TRAINING_ID: &str = "05d6be2c-9286-42e3-89e7-5c37cd418ffb";

const TITLE: &str = "Atemschutz Kurzbericht";
const ACTIVITIES_TEXT: &str = "Tätigkeit";
const ACTIVITY_COBRA_TEXT: &str = "Cobra Cold Cut";
const ACTIVITY_FIRE_FIGHTING_TEXT: &str = "Brandbekämpfung";
const ACTIVITY_LEADING_TEXT: &str = "Führung (als GF/ZF)";
const ACTIVITY_RESCUING_TEXT: &str = "Menschenrettung";
const ACTIVITY_ROOF_OPENING_TEXT: &str = "Dachhautöffnung/Zugangsöffnung";
const ACTIVITY_VENTILATION_TEXT: &str = "Be- und Entlüftungsgerät";
const DATE_TEXT: &str = "Datum";
const DOUBLE_BOTTLE_TEXT: &str = "Zweiflaschengerät(e)";
const DURATION_TEXT: &str = "Gesamtzeit (Min)";
const ISSUES_TEXT: &str = "Probleme";
const OPERATION_TPYE_TEXT: &str = "Alarmart";
const SINGLE_BOTTLE_TEXT: &str = "Einflaschengerät(e)";
const TYPE_TEXT: &str = "Art";
const TYPE_OPERATION_TEXT: &str = "Einsatz";
const TYPE_TRAINING_TEXT: &str = "Übung";

const FIRE_OPERATION_REPORTS_HEADERS: [&str; 10] = [
    "ID",
    "Mitglied",
    DATE_TEXT,
    TYPE_TEXT,
    OPERATION_TPYE_TEXT,
    ACTIVITIES_TEXT,
    DURATION_TEXT,
    ISSUES_TEXT,
    SINGLE_BOTTLE_TEXT,
    DOUBLE_BOTTLE_TEXT,
];

#[derive(Clone, Debug, Default)]
pub struct FireOperationReport {
    pub id: i64,
    pub user: String,
    pub activities: Activities,
    pub date: NaiveDate,
    pub double_bottles: i64,
    pub duration: i64,
    pub issues: String,
    pub operation_type: String,
    pub r#type: Type,
    pub single_bottles: i64,
}

#[derive(Clone, Debug, Default)]
pub enum Type {
    #[default]
    Operation,
    Training,
}

#[derive(Clone, Debug, Default)]
pub struct Activities(Vec<Activity>);

#[derive(Clone, Debug, Default)]
pub enum Activity {
    #[default]
    Cobra,
    FireFighting,
    Leading,
    Rescuing,
    RoofOpening,
    Ventilation,
}

impl FireOperationReport {
    pub fn new_from_report(
        report_type: &response::ReportTypesItem,
        report: &response::Report,
        user: &response::Consumer,
    ) -> Result<Self> {
        let mut fire_operation_report = FireOperationReport {
            id: report.id,
            user: user.stdformat_name.clone(),
            ..Default::default()
        };

        for (field, field_type) in report.fields.iter().zip(report_type.fields.iter()) {
            match field_type.id.as_str() {
                ACTIVITIES_ID => {
                    fire_operation_report.activities =
                        Activities::new(field).context("Failed to parse activity")?;
                }
                DATE_ID => {
                    fire_operation_report.date =
                        parse_date(field).context("Failed to parse date")?;
                }

                DOUBLE_BOTTLES_ID => {
                    fire_operation_report.double_bottles =
                        field.as_i64().context("Failed to parse double bottles")?;
                }
                DURATION_ID => {
                    fire_operation_report.duration =
                        field.as_i64().context("Failed to parse duration")?;
                }
                ISSUES_ID => {
                    fire_operation_report.issues =
                        parse_string(field).context("Failed to parse issues")?;
                }
                OPERATION_TYPE_ID => {
                    fire_operation_report.operation_type =
                        parse_string(field).context("Failed to parse operation type")?;
                }
                TYPE_ID => {
                    let id = parse_string(field).context("Failed to get type id")?;
                    fire_operation_report.r#type = Type::new(&id)?;
                }
                SINGLE_BOTTLES_ID => {
                    fire_operation_report.single_bottles =
                        field.as_i64().context("Failed to parse single bottles")?;
                }
                _ => bail!("Unknown station report type \"{}\"", field_type.name),
            };
        }

        Ok(fire_operation_report)
    }
}

impl Reports for Vec<FireOperationReport> {
    fn new_from_reports(
        report_type: &response::ReportTypesItem,
        reports: response::Reports,
        users: &HashMap<String, response::Consumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut fire_operation_reports: Vec<FireOperationReport> = Vec::default();

        for report in reports.items {
            let user = users
                .get(&report.user_cluster_relation_id.to_string())
                .cloned()
                .unwrap_or_default();
            let station_report = FireOperationReport::new_from_report(report_type, &report, &user)
                .context("Failed to create station report")?;
            fire_operation_reports.push(station_report);
        }

        Ok(fire_operation_reports)
    }

    fn print(self) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(FIRE_OPERATION_REPORTS_HEADERS);
        for report in self {
            table.add_row(vec![
                report.id.to_string(),
                report.user,
                report.date.to_string(),
                report.r#type.to_string(),
                report.operation_type,
                report.activities.to_string(),
                report.duration.to_string(),
                report.issues,
                report.single_bottles.to_string(),
                report.double_bottles.to_string(),
            ]);
        }

        println!("{table}");
    }

    fn write_xlsx(self, path: &Path) -> Result<()> {
        let mut workbook = Workbook::new();
        workbook.read_only_recommended();

        let worksheet = workbook.add_worksheet().set_name(TITLE)?;
        set_table(worksheet, &FIRE_OPERATION_REPORTS_HEADERS, self.len())?;

        for (index, report) in self.into_iter().enumerate() {
            let row = (index + 1) as u32;
            worksheet.write(row, 0, report.id)?;
            worksheet.write(row, 1, report.user)?;
            worksheet.write(row, 2, report.date.to_string())?;
            worksheet.write(row, 3, report.r#type.to_string())?;
            worksheet.write(row, 4, report.operation_type)?;
            worksheet.write(row, 5, report.activities.to_string())?;
            worksheet.write(row, 6, report.duration.to_string())?;
            worksheet.write(row, 7, report.issues)?;
            worksheet.write(row, 8, report.single_bottles.to_string())?;
            worksheet.write(row, 9, report.double_bottles.to_string())?;
        }
        worksheet.autofit();
        workbook.save(path)?;
        Ok(())
    }
}

impl Type {
    pub fn new(id: &str) -> Result<Self> {
        let variant = match id {
            TYPE_OPERATION_ID => Self::Operation,
            TYPE_TRAINING_ID => Self::Training,
            _ => bail!("Unknow type variant \"{}\"", id),
        };

        Ok(variant)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Operation => f.write_str(TYPE_OPERATION_TEXT),
            Type::Training => f.write_str(TYPE_TRAINING_TEXT),
        }
    }
}

impl Activities {
    pub fn new(value: &Value) -> Result<Activities> {
        let activities: Vec<Activity> = value
            .as_array()
            .context("Failed to parse array")?
            .iter()
            .map(|value| parse_string(value).expect("Failed to parse array element as string"))
            .map(|value| Activity::new(&value).expect("Failed to parse activites"))
            .collect();
        Ok(Activities(activities))
    }
}

impl Display for Activities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|activity| activity.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

impl Activity {
    pub fn new(id: &str) -> Result<Self> {
        let variant = match id {
            ACTIVITY_COBRA_ID => Self::Cobra,
            ACTIVITY_FIRE_FIGHTING_ID => Self::FireFighting,
            ACTIVITY_LEADING_ID => Self::Leading,
            ACTIVITY_RESCUING_ID => Self::Rescuing,
            ACTIVITY_ROOF_OPENING_ID => Self::RoofOpening,
            ACTIVITY_VENTILATION_ID => Self::Ventilation,
            _ => bail!("Unknow activity variant \"{}\"", id),
        };

        Ok(variant)
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Activity::Cobra => f.write_str(ACTIVITY_COBRA_TEXT),
            Activity::FireFighting => f.write_str(ACTIVITY_FIRE_FIGHTING_TEXT),
            Activity::Leading => f.write_str(ACTIVITY_LEADING_TEXT),
            Activity::Rescuing => f.write_str(ACTIVITY_RESCUING_TEXT),
            Activity::RoofOpening => f.write_str(ACTIVITY_ROOF_OPENING_TEXT),
            Activity::Ventilation => f.write_str(ACTIVITY_VENTILATION_TEXT),
        }
    }
}
