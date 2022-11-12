use clap::Parser;
use serde::{Deserialize, Serialize};

// defines all different subcommands
#[derive(Parser)]
pub enum Subcommand {
    Start,
    Bridge,
}

/// this defines all the cli commands
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /** this is the config file you want to use,
     *
     * specifying this overwrites the default path
     */
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
    pub log: LogConfig,
    pub security: SecurityConfig,
    pub database: DatabaseConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WebConfig {
    pub port: u16,
    pub address: String,
}
impl Default for WebConfig {
    fn default() -> Self {
        WebConfig {
            port: 4000,
            address: String::from("127.0.0.1"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrontendConfig {
    pub location: String,
}
impl Default for FrontendConfig {
    fn default() -> Self {
        FrontendConfig {
            location: String::from("web"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    pub path: String,
    pub size: f64,
    pub console_level: log::LevelFilter,
    pub file_level: log::LevelFilter,
}
impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            path: String::from("/tmp/intract.log"),
            size: 5.0,
            console_level: log::LevelFilter::Info,
            file_level: log::LevelFilter::Trace,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SecurityConfig {
    /// secret key used to prevent cookies from being decrypted
    /// on the client side
    /// <https://api.rocket.rs/master/rocket/config/struct.SecretKey.html>
    pub secret_key: String,
}
impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            secret_key: String::from("hPRYyVRiMyxpw5sBB1XeCMN1kFsDCqKvBi2QJxBVHQk="),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}
impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            url: String::from("postgres://username:password@localhost/database"),
        }
    }
}

pub fn get_config_file() -> ServerConfig {
    // its ok to unwrap here since we need a working config file anyways
    match &crate::CLI.configpath {
        Some(path) => confy::load_path(path).expect("could not load config"),
        // let confy decide where to find the config
        _ => {
            confy::load("rssc-intract", "config").expect("could not load config from default path")
        }
    }
}

pub fn get_cli() -> Cli {
    Cli::parse()
}
