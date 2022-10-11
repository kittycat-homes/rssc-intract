#![allow(clippy::unnecessary_lazy_evaluations)]
use crate::database as db;
use rand::rngs::OsRng;
use rand::RngCore;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{self, FromRequest, Outcome, Request};

const SESSION_COOKIE_NAME: &str = "session_id";

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
pub struct User(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = &'static str;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        fn validate_session_id(id: &str) -> Outcome<User, &'static str> {
            info!("{}", id);
            // TODO: check database if session id belongs to a user,
            // return Outcome::Success(User("blorbo".to_string()));
            // otherwise forward to login endpoint
            Outcome::Forward(())
        }
        let cookie = request.cookies().get_private(SESSION_COOKIE_NAME);
        match cookie {
            None => Outcome::Forward(()),
            Some(cookie) => validate_session_id(cookie.value()),
        }
    }
}

/**
 * TODO: authenticate more users than just blorbo
 * TODO: support for oauth
 */
fn is_valid_login(username: &str, password: &str) -> bool {
    if username != "blorbo" {
        return false;
    }
    if password != "blorboiscool" {
        return false;
    }
    true
}

pub fn login(jar: &CookieJar<'_>, login: Login) -> Result<(), (Status, &'static str)> {
    if !is_valid_login(login.username, login.password) {
        warn!("failed to authenticate {}", login.username);
        return Err((
            Status::BadRequest,
            "could not match login information with valid info",
        ));
    }
    jar.add_private(Cookie::new(
        SESSION_COOKIE_NAME,
        OsRng.next_u64().to_string(),
    ));
    info!("{} authenticated themselves", login.username);
    Ok(())
}

pub fn add_user(username: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    let salt = OsRng.next_u64().to_string();
    let hash = argon2::hash_encoded(
        &password.into_bytes(),
        &salt.clone().into_bytes(),
        &argon2::Config::default(),
    )?;

    let user = db::user::UserBuilder::default()
        .username(username)
        .hash(Some(hash))
        .salt(Some(salt))
        .build()?;

    // store user if everything is correct
    let _r = db::user::create(user)?;
    Ok(())
}
