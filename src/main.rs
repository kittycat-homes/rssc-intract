use rocket::{figment::Figment, fs::FileServer, Build, Rocket};
use rocket_dyn_templates::Template;
use std::error::Error;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log as logmacro;

#[macro_use]
extern crate derive_builder;

/// this module defines the client api
pub mod api;

/// this module defines config and command line arguments
mod config;

/// this module defines logging
mod log;

/// this is where the actual logic lives
pub mod logic;

/// this is responsible for serving web client
mod web;

/// the admin panel
mod admin;

/// this module defines interactions with the database
pub mod database;

lazy_static! {
    static ref CLI: config::Cli = config::get_cli();
    // the config is dependant on the cli inputs
    // because it gets overwritten by them
    // so we load need to make sure that cli is not dependant on config
    static ref CONF: config::ServerConfig = config::get_config_file();
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    database::run_migrations().expect("couldn't run database migrations"); // updates database

    database::user::delete("johnathan".to_string())?;
    match CLI.subcommand {
        // start the admin panel
        config::Subcommand::Admin => {
            admin::open().await?;
            Ok(())
        }
        // start the server
        config::Subcommand::Start => {
            let _rocket = get_rocket().launch().await?;
            Ok(())
        }
    }
}

/// starts the server
fn get_rocket() -> Rocket<Build> {
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
            .merge(("address", &CONF.web.address))
            .merge(("secret_key", &CONF.security.secret_key))
            .merge(("template_dir", &CONF.frontend.location)),
    )
    // serve api
    .mount("/api", api::routes())
    .mount("/", web::routes())
    .mount(
        "/static",
        FileServer::from(format!("{}/static", &CONF.frontend.location)),
    )
    .attach(Template::fairing())
}
