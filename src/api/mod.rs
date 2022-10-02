use rocket::Route;
mod auth;

/**
(internal) returns all api routes
**/
pub fn routes() -> Vec<Route> {
    [auth::routes()].concat()
}
