mod cli;
mod config;
mod divera;
mod reports;

use anyhow::{bail, Context, Result};
use clap::Parser;
use comfy_table::{ContentArrangement, Table};
use config::Config;
use divera::schema::response::{Consumer, ReportTypes};
use env_logger;
use reports::{
    absent::AbsentReport, fire_operation::FireOperationReport, roster::RosterReport,
    station::StationReport, Reports,
};
use std::{collections::HashMap, fmt::Display, path::Path};

use cli::{Cli, Commands, PrintWriteUpload};

const REPORT_ID_ABSENCES: i64 = 10538;
const REPORT_ID_STATION: i64 = 14307;
const REPORT_ID_ROSTER: i64 = 12112;
const REPORT_ID_FIRE_OPERATION: i64 = 11146;

pub const CONFIG_PATH: &str = "./config.toml";

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or(CONFIG_PATH.to_string());
    let config_path = Path::new(&config_path);

    match cli.command {
        Commands::Init(cmd) => {
            if config_path.exists() {
                bail!("Config already exists. Aborting");
            }

            let config = Config::new(
                cmd.divera_username,
                cmd.divera_password,
                cmd.webdav_username,
                cmd.webdav_password,
                cmd.webdav_directory,
            );
            config.write(config_path)?;
        }
        Commands::ReportTypes => {
            let config = Config::read(config_path)?;
            let login = divera::login(&config.divera.username, &config.divera.password)?;
            let report_types = divera::report_types(&login.user.access_token)?;
            println!("{report_types}");
        }

        Commands::Report(cmd) => {
            let config = Config::read(config_path)?;
            let login = divera::login(&config.divera.username, &config.divera.password)?;
            let all = divera::pull_all(&login.user.access_token)?;
            let users = all.cluster.consumer;
            let report_types = all.cluster.reporttypes;

            match cmd {
                cli::Report::Absences(arguments) => {
                    let reports_name = "absences";
                    let reports: Vec<AbsentReport> = get_reports(
                        REPORT_ID_ABSENCES,
                        &login.user.access_token,
                        &report_types,
                        &users,
                    )
                    .context(format!("Failed to create {reports_name} reports"))?;
                    handle_report_arguments(reports, &config, arguments)
                        .context(format!("Failed handle {reports_name} reports arguments"))?;
                }
                cli::Report::Roster(arguments) => {
                    let reports_name = "roster";
                    let reports: Vec<RosterReport> = get_reports(
                        REPORT_ID_ROSTER,
                        &login.user.access_token,
                        &report_types,
                        &users,
                    )
                    .context(format!("Failed to create {reports_name} reports"))?;
                    handle_report_arguments(reports, &config, arguments)
                        .context(format!("Failed handle {reports_name} reports arguments"))?;
                }
                cli::Report::Station(arguments) => {
                    let reports_name = "station";
                    let reports: Vec<StationReport> = get_reports(
                        REPORT_ID_STATION,
                        &login.user.access_token,
                        &report_types,
                        &users,
                    )
                    .context(format!("Failed to create {reports_name} reports"))?;
                    handle_report_arguments(reports, &config, arguments)
                        .context(format!("Failed handle {reports_name} reports arguments"))?;
                }
                cli::Report::FireOperation(arguments) => {
                    let reports_name = "fire operation";
                    let reports: Vec<FireOperationReport> = get_reports(
                        REPORT_ID_FIRE_OPERATION,
                        &login.user.access_token,
                        &report_types,
                        &users,
                    )
                    .context(format!("Failed to create {reports_name} reports"))?;
                    handle_report_arguments(reports, &config, arguments)
                        .context(format!("Failed handle {reports_name} reports arguments"))?;
                }
            };
        }
    };
    Ok(())
}

fn get_reports<T: Reports>(
    id: i64,
    access_token: &str,
    report_types: &ReportTypes,
    users: &HashMap<String, Consumer>,
) -> Result<T> {
    let reports = divera::reports(&access_token, id).context("Failed to fetch reports")?;
    let report_type = report_types.items.get(&id).cloned().unwrap();
    let reports =
        T::new_from_reports(&report_type, reports, users).context("Failed to create reports")?;

    Ok(reports)
}

fn handle_report_arguments(
    reports: impl Reports,
    config: &Config,
    arguments: PrintWriteUpload,
) -> Result<()> {
    if arguments.print {
        reports.print();
    } else if let Some(output_path) = arguments.write {
        reports
            .write_xlsx(Path::new(&output_path))
            .context("Failed to write fire_operation reports to xlsx")?;
    } else if let Some(file_name) = arguments.upload {
        reports
            .upload(&file_name, config.webdav.clone())
            .context("Failed to upload station reports")?;
    }

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
