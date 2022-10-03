use rocket::{figment::Figment, fs::FileServer};

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
    static ref CLI: config::Cli = config::get_cli();
    // the config is dependant on the cli inputs
    // because it gets overwritten by them
    // so we load need to make sure that cli is not dependant on config
    static ref CONF: config::ServerConfig = config::get_config_file();
}

#[launch]
fn launch() -> _ {
    match CLI.subcommand {
        config::Subcommand::Start => {
            rocket::custom(Figment::from(rocket::Config::default()).merge(("port", CONF.web.port)))
                .mount("/", FileServer::from(&CONF.frontend.location))
                .mount("/api", api::routes())
        }
    }
}
