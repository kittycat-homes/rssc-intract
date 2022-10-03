use clap::{ArgMatches, Args, Command, Parser};
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
    pub config: Option<String>,

    #[command(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub frontend: FrontendConfig,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct FrontendConfig {
    /// relative path to the web frontend location,
    /// this is where your index.html should be
    pub location: String,
}

pub fn get_config_file() -> ServerConfig {
    confy::load("rssc-intract", "config").unwrap()
}
