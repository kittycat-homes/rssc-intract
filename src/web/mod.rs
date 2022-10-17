use rocket::Route;
mod about;
mod auth;
mod settings;
mod share;
mod user;

pub fn routes() -> Vec<Route> {
    [
        routes![user::profile, about::about],
        auth::routes(),
        share::routes(),
        settings::routes(),
    ]
    .concat()
}
