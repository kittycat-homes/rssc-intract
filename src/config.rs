use clap::Parser;
use serde::{Deserialize, Serialize};

// defines all different subcommands
#[derive(Parser)]
pub enum Subcommand {
    Start,
}

/// this defines all the cli commands
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// this is the config file you want to use,
    /// specifying this overwrites the default path
    #[arg(short, long)]
    pub configpath: Option<String>,

    /// the subcommand to use the server with
    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub frontend: FrontendConfig,
    pub web: WebConfig,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FrontendConfig {
    /// relative path to the web frontend location,
    /// this is where your index.html should be
    pub location: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct WebConfig {
    pub port: u16,
}

pub fn get_config_file() -> ServerConfig {
    // its ok to unwrap here since we need a working config file anyways
    match &crate::CLI.configpath {
        Some(path) => confy::load_path(path).unwrap(),
        // let confy decide where to find the config
        _ => confy::load("rssc-intract", "config").unwrap(),
    }
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
