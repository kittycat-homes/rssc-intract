use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;
/// this module defines the client api
mod api;
/// this module defines config and command line arguments
mod config;
/// this is where the actual logic lives
pub mod logic;

#[launch]
fn launch() -> _ {
    match config::cli_get_matches().subcommand() {
        Some(("start", _)) => rocket::build()
            .mount("/", FileServer::from("static/"))
            .mount("/api", api::routes()),
        _ => unreachable!(),
    }
}
