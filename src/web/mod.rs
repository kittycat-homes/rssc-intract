use rocket::Route;
mod about;
mod user;

/// this module does not use the api
/// because the api first needs to
/// get a token through the frontend
mod auth;

pub fn routes() -> Vec<Route> {
    [routes![user::profile, about::about], auth::routes()].concat()
}
