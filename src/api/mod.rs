use rocket::Route;

mod users;

/**
(internal) returns all api routes
**/
pub fn routes() -> Vec<Route> {
    [users::routes()].concat()
}
