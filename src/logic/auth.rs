#![allow(clippy::unnecessary_lazy_evaluations)]
use crate::database as db;
use rand::rngs::OsRng;
use rand::RngCore;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{self, FromRequest, Outcome, Request};
use std::error::Error;

const SESSION_COOKIE_NAME: &str = "session_id";

#[derive(thiserror::Error, Debug)]
pub enum AuthenticationError {
    #[error("validity of credentials could not be verified")]
    IncorrectCredentials,
    #[error("missing hash")]
    MissingHash,
    #[error("could not validate session")]
    SessionError,
}

#[derive(FromForm)]
pub struct Login<'r> {
    username: &'r str,
    password: &'r str,
}

/**
 * a fully authenticated user
 * [for security reasons](https://rocket.rs/v0.5-rc/guide/requests/#guard-transparency) there should not be any additonal constructors for this
 */
#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub user: db::user::User,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = AuthenticationError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Session, Self::Error> {
        fn validate_session_id(id: &str) -> Outcome<Session, AuthenticationError> {
            // check if id is valid by getting a user
            let user = db::sessionid::belongs_to(id.to_string());
            match user {
                Err(_) => Outcome::Forward(()),
                Ok(user) => Outcome::Success(Session {
                    user,
                    id: id.to_string(),
                }),
            }
        }

        let cookie = request.cookies().get_private(SESSION_COOKIE_NAME);
        match cookie {
            None => Outcome::Forward(()),
            Some(cookie) => validate_session_id(cookie.value()),
        }
    }
}

/**
 * TODO: support for oauth
 */
fn is_valid_login(username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let user = db::user::get(username.to_string())?;
    // hash of already existing user
    let uhash = user.hash.ok_or(AuthenticationError::MissingHash)?;

    let valid = argon2::verify_encoded(&uhash, &password.to_string().into_bytes())?;
    Ok(valid)
}

pub fn login(jar: &CookieJar<'_>, login: Login) -> Result<(), Box<dyn Error>> {
    if !is_valid_login(login.username, login.password)? {
        warn!("failed to authenticate {}", login.username);
        return Err(AuthenticationError::IncorrectCredentials.into());
    }
    info!("found valid login");

    // cryptographically secure random number
    let id = OsRng.next_u64();

    // create builder for user id
    let session = db::sessionid::SessionIDBuilder::default()
        .username(login.username.to_string())
        .id(id.to_string())
        .last_active(std::time::SystemTime::now())
        .build()?;

    trace!("created session builder: id: {}", session.id);

    // save userid to database
    let _dbres = db::sessionid::create(session)?;
    jar.add_private(Cookie::new(SESSION_COOKIE_NAME, id.to_string()));
    info!("{} authenticated themselves", login.username);
    Ok(())
}

/// add a new user to the database
pub fn add_user(username: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    let gen = generate_hash(password)?;

    let user = db::user::UserBuilder::default()
        .username(username)
        .hash(Some(gen.0))
        .salt(Some(gen.1))
        .build()?;

    // store user if everything is correct
    let _r = db::user::create(user)?;
    Ok(())
}

/// change a users password
pub fn change_password(
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let gen = generate_hash(password)?;
    let update = db::user::UpdateUserBuilder::default()
        .hash(Some(gen.0))
        .salt(Some(gen.1))
        .build()?;
    let _user = db::user::update(username, update)?;
    Ok(())
}

/**
 * generate hash and salt for this password
 */
fn generate_hash(password: String) -> Result<(String, String), Box<dyn std::error::Error>> {
    let salt = OsRng.next_u64().to_string();
    let hash = argon2::hash_encoded(
        &password.into_bytes(),
        &salt.clone().into_bytes(),
        &argon2::Config::default(),
    )?;
    Ok((hash, salt))
}
