use rocket::{
    figment::Figment,
    shield::{Referrer, Shield},
    Build, Rocket,
};
use std::error::Error;

extern crate argon2;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log as logmacro;

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate diesel;

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
    match CLI.subcommand {
        // start the admin panel
        config::Subcommand::Bridge => {
            database::run_migrations().expect("couldn't run database migrations"); // updates database
            let res = admin::main_menu().open();
            match &res {
                Ok(_) => {
                    println!("ok!");
                    res
                }
                Err(e) => {
                    println!("{}", e);
                    res
                }
            }
        }
        // start the server
        config::Subcommand::Start => {
            database::run_migrations().expect("couldn't run database migrations"); // updates database
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
            .merge(("secret_key", &CONF.security.secret_key)),
    )
    // serve api
    .mount("/api", api::routes())
    .mount("/", web::routes())
    .attach(Shield::default().enable(Referrer::NoReferrer))
    .attach(web::assets::fairing())
    .mount("/static", web::assets::routes())
}
