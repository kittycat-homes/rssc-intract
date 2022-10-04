use crate::logic::auth::{authenticate_user, UserAuthData};
use rocket::{http::Status, serde::json::Json, Route};

pub fn routes() -> Vec<Route> {
    routes![login]
}

/**
TODO
if given a correct username and password this returns a json web token
**/
#[post("/v1/auth/login", format = "json", data = "<user_auth_data>")]
pub async fn login(user_auth_data: Json<UserAuthData>) -> Result<String, (Status, &'static str)> {
    authenticate_user(user_auth_data.into_inner()).await
}
