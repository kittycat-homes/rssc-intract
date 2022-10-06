use chrono::prelude::*;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};

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
pub struct User(usize);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        request
            .cookies()
            .get_private("session_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(User)
            .or_forward(())
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

/**
 * should generate a unique session id
 */
fn generate_session_id(username: &str) -> String {
    format!("{}{}", username, Utc::now().timestamp())
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
        "session_id",
        generate_session_id(login.username),
    ));
    info!("{} authenticated themselves", login.username);
    Ok(())
}
