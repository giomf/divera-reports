mod cli;
mod config;
mod divera;

use std::path::Path;

use anyhow::{bail, Result};
use clap::Parser;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Init(cmd) => {
            if Path::new(config::CONFIG_PATH).exists() {
                bail!("Config already exists. Aborting");
            }

            let config = Config::new(cmd.divera_username, cmd.divera_password);
            config.write();
        }
        Cli::Test(_) => {
            let config = Config::read();
            divera::login(config.divera.username, config.divera.password)?;
        }
    };

    Ok(())
}
