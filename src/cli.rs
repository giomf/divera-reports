use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Divera reports", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Config path
    #[arg(global = true, short, long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Subcommands of the application
#[derive(Subcommand, Debug)]
pub enum Commands {
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

    /// Username for webdav server
    #[arg(long)]
    pub webdav_username: String,
    /// Password for webdav server
    #[arg(long)]
    pub webdav_password: String,
    /// Root directory for webdav server
    #[arg(long)]
    pub webdav_directory: String,
}

#[derive(Debug, Subcommand)]
pub enum Report {
    /// Absences reports
    Absences(PrintWriteUpload),
    /// Roster reports
    Roster(PrintWriteUpload),
    /// Station reports
    Station(PrintWriteUpload),
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct PrintWriteUpload {
    /// Prints the reports in a table format
    #[arg(long)]
    pub print: bool,

    /// Writes the reports to an xlsx file
    #[arg(long)]
    pub write: Option<String>,

    /// Exports the report as xlsx and upload it to webdav server
    #[arg(long)]
    pub upload: Option<String>,
}
