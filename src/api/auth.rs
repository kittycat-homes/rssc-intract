use crate::logic::auth::{authenticate_token, authenticate_user, Claims, ClientAuthData, Token};
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

#[post("/v1/auth/whoami", format = "json", data = "<key>")]
pub async fn whoami(key: Json<Token>) -> Result<Json<Claims>, (Status, String)> {
    match authenticate_token(&key.into_inner().value) {
        Ok(t) => Ok(Json(t.claims)),
        Err(e) => Err(e),
    }
}
