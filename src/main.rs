mod cli;
mod config;

use std::path::Path;

use clap::Parser;

use cli::Cli;
use config::Config;

fn main() {
    let cli = Cli::parse();

    match cli {
        Cli::Init(cmd) => {
            if Path::new(config::CONFIG_PATH).exists() {
                println!("Config already exists. Aborting");
                return;
            }

            let config = Config::new(cmd.divera_username, cmd.divera_password);
            config.write();
        }
    };
}
