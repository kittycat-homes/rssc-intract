use rocket::{figment::Figment, fs::FileServer};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

/// this module defines the client api
mod api;

/// this module defines config and command line arguments
mod config;

/// this module defines logging
mod log;

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
            // initialize logging
            log::init_log(
                &CONF.log.path,
                CONF.log.size,
                CONF.log.console_level,
                CONF.log.file_level,
            )
            .expect("log file could not be made");

            rocket::custom(
                Figment::from(rocket::Config::default())
                    // settings for rocket
                    .merge(("port", &CONF.web.port))
                    .merge(("address", &CONF.web.address)),
            )
            // serve frontend
            .mount("/", FileServer::from(&CONF.frontend.location))
            // serve api
            .mount("/api", api::routes())
        }
    }
}
