use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Divera reports", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    /// Initialize the config
    Init(Init),
    ///Prints available report types
    ReportTypes,
    /// Prints or writes reports
    #[command(subcommand)]
    Report(Report),
}

#[derive(Debug, Args)]
pub struct Init {
    /// Username for divera247
    #[arg(long)]
    pub divera_username: String,
    /// Password for divera247
    #[arg(long)]
    pub divera_password: String,
}

#[derive(Debug, Subcommand)]
pub enum Report {
    /// Absences reports
    Absences(PrintOrWrite),
    /// Roster reports
    Roster(PrintOrWrite),
    /// Station reports
    Station(PrintOrWrite),
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct PrintOrWrite {
    /// Prints the reports in a table format
    #[arg(long)]
    pub print: bool,

    /// Writes the reports to an xlsx file
    #[arg(long)]
    pub write: Option<String>,
}
