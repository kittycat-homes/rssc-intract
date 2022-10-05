use crate::logic::auth::{authenticate_user, Claims, ClientAuthData, Token, User};
use rocket::{http::Status, serde::json::Json, Route};

pub fn routes() -> Vec<Route> {
    routes![login, whoami]
}

/**
TODO
if given a correct username and password this returns a json web token
**/
#[post("/v1/auth/login", format = "json", data = "<client_auth_data>")]
pub async fn login(client_auth_data: Json<ClientAuthData>) -> Result<String, (Status, String)> {
    authenticate_user(client_auth_data.into_inner()).await
}

#[get("/v1/auth/whoami")]
pub async fn whoami(_user: User) -> &'static str {
    "hello"
}
