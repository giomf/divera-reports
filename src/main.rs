mod cli;
mod config;
mod divera;
mod reports;

use anyhow::{bail, Context, Result};
use clap::Parser;
use comfy_table::{ContentArrangement, Table};
use divera::report_types;
use env_logger;
use reports::{
    absent::{create_absent_reports, print_absent_reports},
    roster::{create_roster_reports, print_roster_reports},
    station::{create_station_reports, print_station_reports},
};
use std::{fmt::Display, path::Path};

use cli::Cli;
use config::Config;

const REPORT_ID_ABSENCES: i64 = 10538;
const REPORT_ID_STATION: i64 = 14307;
const REPORT_ID_ROSTER: i64 = 12112;
const REPORT_ID_FIRE_OPERATION: i64 = 11146;

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli {
        Cli::Init(cmd) => {
            if Path::new(config::CONFIG_PATH).exists() {
                bail!("Config already exists. Aborting");
            }

            let config = Config::new(cmd.divera_username, cmd.divera_password);
            config.write();
        }
        Cli::ReportTypes => {
            let config = Config::read();
            let login = divera::login(config.divera.username, config.divera.password)?;
            let report_types = divera::report_types(&login.user.access_token)?;
            println!("{report_types}");
        }

        Cli::Report(cmd) => {
            let config = Config::read();
            let login = divera::login(config.divera.username, config.divera.password)?;
            dbg!(&login);
            let all = divera::pull_all(&login.user.access_token)?;
            let users = all.cluster.consumer;
            let report_types = all.cluster.reporttypes;

            match cmd {
                cli::Report::Absences {} => {
                    let reports = divera::reports(&login.user.access_token, REPORT_ID_ABSENCES)?;
                    let report_type = report_types
                        .items
                        .get(&REPORT_ID_ABSENCES)
                        .cloned()
                        .unwrap();
                    let absent_reports = create_absent_reports(report_type, reports, users)?;
                    print_absent_reports(absent_reports);
                }
                cli::Report::Roster {} => {
                    let reports = divera::reports(&login.user.access_token, REPORT_ID_ROSTER)?;
                    let report_type = report_types.items.get(&REPORT_ID_ROSTER).cloned().unwrap();
                    let roster_reports = create_roster_reports(report_type, reports, users)
                        .context("Failed to create roster reports")?;
                    print_roster_reports(roster_reports);
                }
                cli::Report::Station {} => {
                    let reports = divera::reports(&login.user.access_token, REPORT_ID_STATION)?;
                    let report_type = report_types.items.get(&REPORT_ID_STATION).cloned().unwrap();
                    let station_reports = create_station_reports(report_type, reports, users)
                        .context("Failed to create station reports")?;
                    print_station_reports(station_reports);
                }
                cli::Report::FireOperation {} => {
                    let reports =
                        divera::reports(&login.user.access_token, REPORT_ID_FIRE_OPERATION)?;
                    let report_type = report_types
                        .items
                        .get(&REPORT_ID_FIRE_OPERATION)
                        .cloned()
                        .unwrap();
                    dbg!(report_type);
                }
            }
        }
    };

    Ok(())
}

impl Display for divera::schema::response::ReportTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table
            .set_header(vec!["ID", "Name", "Description"])
            .set_content_arrangement(ContentArrangement::Dynamic);
        for (id, item) in self.items.iter() {
            table.add_row(vec![
                id.to_string(),
                item.name.clone().lines().collect(),
                item.description.clone().lines().collect(),
            ]);
        }
        f.write_str(&table.to_string())
    }
}
