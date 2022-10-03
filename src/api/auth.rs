use crate::logic::auth::UserAuthData;
use rocket::{serde::json::Json, Route};

pub fn routes() -> Vec<Route> {
    routes![login]
}

/**
TODO
if given a correct username and password this returns a json web token
**/
#[post("/v1/auth/login", format = "json", data = "<user_auth_data>")]
pub fn login(user_auth_data: Json<UserAuthData>) -> Json<UserAuthData> {
    user_auth_data
}
