use rocket::Route;
pub mod assets;
mod auth;
pub mod components;
pub mod language;
mod my_data;
mod robots;
mod settings;
mod subscribe;
mod user;

pub fn routes() -> Vec<Route> {
    [
        auth::routes(),
        settings::routes(),
        user::routes(),
        my_data::routes(),
        robots::routes(),
        subscribe::routes(),
    ]
    .concat()
}
