use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "Divera reports", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    /// Initialize the config
    Init(Init),

    ReportTypes,

    #[command(subcommand)]
    Report(Report),
}

#[derive(Debug, Parser)]
pub struct Init {
    /// Username for divera247
    #[clap(long)]
    pub divera_username: String,
    /// Password for divera247
    #[clap(long)]
    pub divera_password: String,
}

#[derive(Debug, Subcommand)]
pub enum Report {
    Absences {},
    Roster {},
    Station {},
    FireOperation {},
}
