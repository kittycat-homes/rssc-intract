#![allow(clippy::unnecessary_lazy_evaluations)]
use crate::database as db;
use rand::distributions::Alphanumeric;
use rand::rngs::OsRng;
use rand::{thread_rng, Rng, RngCore};
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
    #[error("password is missing!")]
    NoPassword,
    #[error("username contains disallowed characters")]
    EvilChars,
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

/// returns true if login is valid
pub fn is_valid_login(username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
    let user = db::user::get(username.to_string())?;
    // hash of already existing user
    let uhash = user.hash.ok_or(AuthenticationError::MissingHash)?;

    let valid = argon2::verify_encoded(&uhash, &password.to_string().into_bytes())?;
    Ok(valid)
}

pub fn logout(jar: &CookieJar<'_>, session: &Session) -> Result<(), Box<dyn Error>> {
    // delete login cookie
    jar.remove_private(Cookie::named(SESSION_COOKIE_NAME));

    // make session invalid by deleting session from db
    db::sessionid::delete(session.id.to_string())?;
    Ok(())
}

pub fn login(jar: &CookieJar<'_>, login: Login) -> Result<(), Box<dyn Error>> {
    if !is_valid_login(login.username, login.password)? {
        warn!("failed to authenticate {}", login.username);
        return Err(AuthenticationError::IncorrectCredentials.into());
    }
    info!("found valid login");

    let mut rng = thread_rng();
    let id: String = (0..64).map(|_| rng.sample(Alphanumeric) as char).collect();

    // create builder for user id
    let session = db::sessionid::SessionIDBuilder::default()
        .username(login.username.to_string())
        .id(id.to_owned())
        .last_active(std::time::SystemTime::now())
        .build()?;

    trace!("created session builder: id: {}", session.id);

    // save userid to database
    let _dbres = db::sessionid::create(session)?;
    jar.add_private(Cookie::new(SESSION_COOKIE_NAME, id));
    info!("{} authenticated themselves", login.username);
    Ok(())
}

/// add a new user to the database
pub fn add_user(username: String, password: String) -> Result<(), Box<dyn std::error::Error>> {
    if password.is_empty() {
        return Err(AuthenticationError::NoPassword)?;
    }

    // we need to not have ats in usernames since this is how we distinguish
    // between remote users and local users
    if username.contains('@') {
        return Err(AuthenticationError::EvilChars)?;
    }

    let gen = generate_hash(password)?;

    let user = db::user::UserBuilder::default()
        .username(username)
        .hash(Some(gen))
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
    if password.is_empty() {
        return Err(AuthenticationError::NoPassword)?;
    }

    let gen = generate_hash(password)?;

    // delete all current sessions
    db::sessionid::delete_user_sessionids(username.to_owned())?;

    // update user
    let update = db::user::UpdateUserBuilder::default()
        .hash(Some(gen))
        .build()?;
    let _user = db::user::update(username, update)?;
    Ok(())
}

/**
 * generate hash and salt for this password
 */
fn generate_hash(password: String) -> Result<String, Box<dyn std::error::Error>> {
    let salt = OsRng.next_u64().to_string();
    let hash = argon2::hash_encoded(
        &password.into_bytes(),
        &salt.into_bytes(),
        &argon2::Config::default(),
    )?;
    Ok(hash)
}

#[cfg(test)]
mod tests {
    use crate::logic::auth::generate_hash;

    #[test]
    fn test_hashgen() {
        let password = "hunter2".to_string();
        let hash = generate_hash(password.to_string()).unwrap();
        let valid = argon2::verify_encoded(&hash, &password.as_bytes()).unwrap();

        // test to see if hash is valid
        assert!(valid);
        assert_ne!(hash, password);
    }
}
