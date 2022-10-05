use crate::CONF;
use jsonwebtoken::{self as jwt, get_current_timestamp};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

lazy_static! {
    // set secret for encoding json web token
    static ref JWTKENCODE: jwt::EncodingKey = jwt::EncodingKey::from_secret(CONF.jwt.secret.as_ref());
    static ref JWTKDECODE: jwt::DecodingKey = jwt::DecodingKey::from_secret(CONF.jwt.secret.as_ref());
}

/**
 * an existing and authenticed user
 * provides [guard transparency](https://rocket.rs/v0.5-rc/guide/requests/#guard-transparency)
 */
pub struct User(Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("token") {
            None => Outcome::Failure((Status::BadRequest, "no key provided".to_string())),
            Some(token) => {
                info!("{}", token);
                match authenticate_token(token) {
                    Err(e) => Outcome::Failure((e.0, e.1)),
                    Ok(r) => Outcome::Success(User(r.claims)),
                }
            }
        }
    }
}

/**
 * scopes that define what a client is allowed to do
 */
#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Scope {
    Read,
    Write,
}

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub userid: String,
    pub scopes: Vec<Scope>,
    pub exp: u64,
}

impl Claims {
    pub fn new(userid: String, scopes: Vec<Scope>) -> Claims {
        Claims {
            userid,
            scopes,
            exp: get_current_timestamp() + CONF.jwt.ttl,
        }
    }
}

/**
data used to log a user in with password and userid
**/
#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct UserAuthData {
    pub userid: String,
    pub pass: String,
}

/**
 * used to define scopes when authenticating a client
 */
#[derive(Deserialize, Serialize)]
pub struct ClientAuthData {
    scopes: Vec<Scope>,
    user: UserAuthData,
}

/// get set of valid users
fn get_valid_users() -> HashSet<UserAuthData> {
    //TODO do not just authenticate zork
    let zork = UserAuthData {
        userid: "zork".to_string(),
        pass: "zorkrules".to_string(),
    };
    HashSet::from([zork])
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub value: String,
}

/**
*initial authentication with username and password
*TODO returns encoded token if credentials are true
*/
pub async fn authenticate_user(data: ClientAuthData) -> Result<String, (Status, String)> {
    if !get_valid_users().contains(&data.user) {
        return Err((
            Status::Unauthorized,
            "failed to authenticate user with these credentials".to_string(),
        ));
    }
    jwt::encode(
        &jwt::Header::default(),
        &Claims::new(data.user.userid, data.scopes),
        &JWTKENCODE,
    )
    .map_err(|e| (Status::InternalServerError, e.to_string()))
}

/**
 * authentication with jsonwebtoken, returns error if authentication failed
 * TODO check if user exists in database
*/
pub fn authenticate_token(token: &str) -> Result<jwt::TokenData<Claims>, (Status, String)> {
    jwt::decode::<Claims>(token, &JWTKDECODE, &jwt::Validation::default())
        .map_err(|e| (Status::Unauthorized, e.to_string()))
}
