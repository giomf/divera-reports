mod cli;
mod config;
mod divera;
mod reports;

use anyhow::{bail, Context, Result};
use clap::Parser;
use comfy_table::{ContentArrangement, Table};
use env_logger;
use reports::{absent::AbsentReport, roster::RosterReport, station::StationReport, Reports};
use std::{fmt::Display, path::Path};

use cli::Cli;
use config::Config;

const REPORT_ID_ABSENCES: i64 = 10538;
const REPORT_ID_STATION: i64 = 14307;
const REPORT_ID_ROSTER: i64 = 12112;

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli {
        Cli::Init(cmd) => {
            if Path::new(config::CONFIG_PATH).exists() {
                bail!("Config already exists. Aborting");
            }

            let config = Config::new(
                cmd.divera_username,
                cmd.divera_password,
                cmd.webdav_username,
                cmd.webdav_password,
                cmd.webdav_directory,
            );
            config.write()?;
        }
        Cli::ReportTypes => {
            let config = Config::read()?;
            let login = divera::login(config.divera.username, config.divera.password)?;
            let report_types = divera::report_types(&login.user.access_token)?;
            println!("{report_types}");
        }

        Cli::Report(cmd) => {
            let config = Config::read()?;
            let login = divera::login(config.divera.username, config.divera.password)?;
            let all = divera::pull_all(&login.user.access_token)?;
            let users = all.cluster.consumer;
            let report_types = all.cluster.reporttypes;

            match cmd {
                cli::Report::Absences(arguments) => {
                    let reports = divera::reports(&login.user.access_token, REPORT_ID_ABSENCES)?;
                    let report_type = report_types
                        .items
                        .get(&REPORT_ID_ABSENCES)
                        .cloned()
                        .unwrap();
                    let absent_reports =
                        Vec::<AbsentReport>::new_from_reports(report_type, reports, users)
                            .context("Failed to create absent reports")?;
                    if arguments.print {
                        absent_reports.print();
                    } else if let Some(output_path) = arguments.write {
                        absent_reports
                            .write_xlsx(Path::new(&output_path))
                            .context("Failed to write absent reports to xlsx")?;
                    } else if let Some(file_name) = arguments.upload {
                        absent_reports
                            .upload(&file_name, config.webdav)
                            .context("Failed to upload station reports")?;
                    }
                }
                cli::Report::Roster(arguments) => {
                    let reports = divera::reports(&login.user.access_token, REPORT_ID_ROSTER)?;
                    let report_type = report_types.items.get(&REPORT_ID_ROSTER).cloned().unwrap();
                    let roster_reports =
                        Vec::<RosterReport>::new_from_reports(report_type, reports, users)
                            .context("Failed to create roster reports")?;
                    if arguments.print {
                        roster_reports.print();
                    } else if let Some(output_path) = arguments.write {
                        roster_reports
                            .write_xlsx(Path::new(&output_path))
                            .context("Failed to write roster reports to xlsx")?;
                    } else if let Some(file_name) = arguments.upload {
                        roster_reports
                            .upload(&file_name, config.webdav)
                            .context("Failed to upload station reports")?;
                    }
                }
                cli::Report::Station(arguments) => {
                    let reports = divera::reports(&login.user.access_token, REPORT_ID_STATION)?;
                    let report_type = report_types.items.get(&REPORT_ID_STATION).cloned().unwrap();
                    let station_reports =
                        Vec::<StationReport>::new_from_reports(report_type, reports, users)
                            .context("Failed to create station reports")?;
                    if arguments.print {
                        station_reports.print();
                    } else if let Some(output_path) = arguments.write {
                        station_reports
                            .write_xlsx(Path::new(&output_path))
                            .context("Failed to write station reports to xlsx")?;
                    } else if let Some(file_name) = arguments.upload {
                        station_reports
                            .upload(&file_name, config.webdav)
                            .context("Failed to upload station reports")?;
                    }
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
