use clap::{ArgMatches, Parser};
use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

/// this module defines the client api
mod api;

/// this module defines config and command line arguments
mod config;

/// this is where the actual logic lives
pub mod logic;

lazy_static! {
    static ref CLI: config::Cli = config::Cli::parse();
    static ref CONF: config::ServerConfig = config::get_config_file();
}

#[launch]
fn launch() -> _ {
    match CLI.subcommand {
        config::Subcommand::Start => rocket::build()
            .mount("/", FileServer::from(&CONF.frontend.location))
            .mount("/api", api::routes()),
    }
}
