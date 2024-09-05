use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Divera reports", long_about = None)]
#[clap(propagate_version = true)]
pub enum Cli {
    /// Initialize the config
    Init(Init),
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
