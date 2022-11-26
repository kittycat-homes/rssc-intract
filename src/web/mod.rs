use rocket::Route;
pub mod assets;
mod auth;
pub mod components;
pub mod language;
mod my_data;
mod robots;
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
        my_data::routes(),
        robots::routes(),
    ]
    .concat()
}
