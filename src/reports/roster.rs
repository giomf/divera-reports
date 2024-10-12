use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, bail, Context, Result};
use comfy_table::{ContentArrangement, Table};

use super::{parse_string, Reports};
use crate::divera::schema::response;

const TYPE_ID: &str = "ab71921a-70b5-46de-b198-e342c50fe262";
const TOPIC_ID: &str = "24868bfa-c903-437f-9959-f7ea888e0145";
const PARTICIPATION_ID: &str = "d8049a3a-407c-480f-93f8-6736a27e9d6e";
const TIMESCOPE_ID: &str = "2fa25c9d-8ed2-4a05-ab19-7464a4098572";
const DESCRIPTION_ID: &str = "2cefd98b-9ea5-4329-b657-7a2a74483c51";
const POTENTIAL_DATE_ID: &str = "d6370fa1-64e4-4108-aa73-6ee528aa7210";
const TYPE_TEXT: &str = "Art";
const TOPIC_TEXT: &str = "Thema";
const PARTICIPATION_TEXT: &str = "Mitgestaltung";
const TIMESCOPE_TEXT: &str = "Zeitumfang";
const DESCRIPTION_TEXT: &str = "Beschreibung";
const POTENTIAL_DATE_TEXT: &str = "Mögliches Datum/Monat";

const TYPE_TRAINING_ID: &str = "1cfa8920-be7b-4e2b-be6b-4ba16bde6aa5";
const TYPE_EVENT_ID: &str = "36f1d684-4e00-4fbf-8e3a-ff5a9bf2e931";
const TYPE_TRAINING_TEXT: &str = "Übungsdienst";
const TYPE_EVENT_TEXT: &str = "Veranstaltung";

const PARTICIPATION_RESPONSIBLE_ID: &str = "bbe4ffb2-142e-40da-b812-298004be4bdc";
const PARTICIPATION_HELPING_ID: &str = "57e60afd-be43-48b2-ba73-d092f999b91c";
const PARTICIPATION_RESPONSIBLE_TEXT: &str = "Verantwortlich";
const PARTICIPATION_HELPING_TEXT: &str = "Helfend";

const TIMESCOPE_HALF_ID: &str = "84f61e5c-8584-4e25-86bf-40caf973290f";
const TIMESCOPE_FULL_ID: &str = "5323ebf6-bfa9-426c-8c5b-84f25a30e7d7";
const TIMESCOPE_BOTH_ID: &str = "b90fa9df-ee7e-48fd-b356-6d5c4146e9c7";
const TIMESCOPE_OTHER_ID: &str = "27e014a4-c455-4108-80e7-5fe8640283bb";
const TIMESCOPE_HALF_TEXT: &str = "Mittwoch abend";
const TIMESCOPE_FULL_TEXT: &str = "Samstag";
const TIMESCOPE_BOTH_TEXT: &str = "Kombi (Mi+Sa)";
const TIMESCOPE_OTHER_TEXT: &str = "Außer der Reihe";

const ROSTER_REPORTS_HEADERS: [&str; 8] = [
    "ID",
    "Mitglied",
    TYPE_TEXT,
    PARTICIPATION_TEXT,
    TIMESCOPE_TEXT,
    POTENTIAL_DATE_TEXT,
    TOPIC_TEXT,
    DESCRIPTION_TEXT,
];

#[derive(Clone, Debug, Default)]
pub struct RosterReport {
    pub id: i64,
    pub user: String,
    pub r#type: Type,
    pub topic: String,
    pub participation: Option<Participation>,
    pub time_scope: Option<TimeScope>,
    pub description: String,
    pub potential_date: String,
}

#[derive(Clone, Debug, Default)]
pub enum Type {
    #[default]
    Training,
    Event,
}
#[derive(Clone, Debug, Default)]
pub enum Participation {
    #[default]
    Responsible,
    Helping,
}
#[derive(Clone, Debug, Default)]
pub enum TimeScope {
    #[default]
    Half,
    Full,
    Both,
    Other,
}

impl RosterReport {
    pub fn new_from_report(
        report_type: &response::ReportTypesItem,
        report: &response::Report,
        user: &response::Consumer,
    ) -> Result<Self> {
        let mut roster_report = RosterReport {
            id: report.id,
            user: user.stdformat_name.clone(),
            ..Default::default()
        };

        for (field, field_type) in report.fields.iter().zip(report_type.fields.iter()) {
            match field_type.id.as_str() {
                DESCRIPTION_ID => {
                    roster_report.description =
                        parse_string(field).context("Failed to parse description")?;
                }
                PARTICIPATION_ID => {
                    let options = field_type
                        .options
                        .clone()
                        .ok_or_else(|| anyhow!("Failed to get participation options"))?;
                    let id = parse_string(field).context("Failed to get participation id")?;
                    roster_report.participation = Participation::new(options, &id)
                        .context("Failed to create participation")?;
                }
                POTENTIAL_DATE_ID => {
                    roster_report.potential_date =
                        parse_string(field).context("Failed to get potential date")?;
                }
                TIMESCOPE_ID => {
                    let options = field_type
                        .options
                        .clone()
                        .ok_or_else(|| anyhow!("Failed to get timescope options"))?;
                    let id = parse_string(field).context("Failed to get timescope id")?;
                    roster_report.time_scope =
                        TimeScope::new(options, &id).context("Failed to create timescope")?;
                }
                TOPIC_ID => {
                    roster_report.topic = parse_string(field).context("Failed to get topic")?;
                }
                TYPE_ID => {
                    let options = field_type
                        .options
                        .clone()
                        .ok_or_else(|| anyhow!("Failed to get type options"))?;
                    let id = parse_string(field).context("Failed to get type id")?;
                    roster_report.r#type =
                        Type::new(options, &id).context("Failed to create type")?;
                }
                _ => bail!("Unknown roster report type id \"{}\"", field_type.name),
            };
        }
        Ok(roster_report)
    }
}

impl Reports for Vec<RosterReport> {
    fn new_from_reports(
        report_type: response::ReportTypesItem,
        reports: response::Reports,
        users: HashMap<String, response::Consumer>,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut roster_reports: Vec<RosterReport> = Vec::default();
        for report in reports.items {
            let user = users
                .get(&report.user_cluster_relation_id.to_string())
                .cloned()
                .unwrap_or_default();
            roster_reports.push(RosterReport::new_from_report(&report_type, &report, &user)?);
        }
        Ok(roster_reports)
    }

    fn print(self) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);
        table.set_header(ROSTER_REPORTS_HEADERS);
        for report in self {
            table.add_row(vec![
                report.id.to_string(),
                report.user,
                report.r#type.to_string(),
                report
                    .participation
                    .map_or(String::default(), |participation| participation.to_string()),
                report
                    .time_scope
                    .map_or(String::default(), |time_scope| time_scope.to_string()),
                report.potential_date,
                report.topic,
                report.description,
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
            TYPE_EVENT_ID => Self::Event,
            TYPE_TRAINING_ID => Self::Training,
            _ => bail!("Unknow type variant \"{}\"", r#type.id),
        };

        Ok(variant)
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Training => f.write_str(TYPE_TRAINING_TEXT),
            Type::Event => f.write_str(TYPE_EVENT_TEXT),
        }
    }
}

impl Participation {
    pub fn new(types: Vec<response::ReportTypesItemFieldOption>, id: &str) -> Result<Option<Self>> {
        if let Some(r#type) = types.iter().find(|r#type| r#type.id == id) {
            let variant = match r#type.id.as_str() {
                PARTICIPATION_HELPING_ID => Self::Helping,
                PARTICIPATION_RESPONSIBLE_ID => Self::Responsible,
                _ => bail!("Unknow type variant \"{}\"", r#type.id),
            };
            return Ok(Some(variant));
        } else {
            return Ok(None);
        }
    }
}

impl Display for Participation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Participation::Responsible => f.write_str(PARTICIPATION_RESPONSIBLE_TEXT),
            Participation::Helping => f.write_str(PARTICIPATION_HELPING_TEXT),
        }
    }
}

impl TimeScope {
    pub fn new(types: Vec<response::ReportTypesItemFieldOption>, id: &str) -> Result<Option<Self>> {
        if let Some(r#type) = types.iter().find(|r#type| r#type.id == id) {
            let variant = match r#type.id.as_str() {
                TIMESCOPE_BOTH_ID => Self::Both,
                TIMESCOPE_FULL_ID => Self::Full,
                TIMESCOPE_HALF_ID => Self::Half,
                TIMESCOPE_OTHER_ID => Self::Other,
                _ => bail!("Unknow type variant \"{}\"", r#type.id),
            };

            return Ok(Some(variant));
        } else {
            return Ok(None);
        }
    }
}

impl Display for TimeScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeScope::Half => f.write_str(TIMESCOPE_HALF_TEXT),
            TimeScope::Full => f.write_str(TIMESCOPE_FULL_TEXT),
            TimeScope::Both => f.write_str(TIMESCOPE_BOTH_TEXT),
            TimeScope::Other => f.write_str(TIMESCOPE_OTHER_TEXT),
        }
    }
}
