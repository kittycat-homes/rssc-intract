use rocket::Route;
mod auth;
pub mod components;
pub mod language;
mod settings;
mod share;
mod user;

pub fn routes() -> Vec<Route> {
    [
        routes![user::profile],
        auth::routes(),
        share::routes(),
        settings::routes(),
        user::routes(),
    ]
    .concat()
}
