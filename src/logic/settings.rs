#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::database as db;
use crate::logic::auth;
use rocket::http::{Cookie, CookieJar};
use rocket::FromForm;
use std::error::Error;

#[derive(thiserror::Error, Debug)]
enum SettingsError {
    #[error("login could not be validated")]
    LoginInvalid,
    #[error("no password set")]
    NoPassword,
}

/**
 * struct used to update settings
 * settings that should not be changed should be None
 */
#[derive(FromForm)]
pub struct UserSettings<'r> {
    displayname: Option<&'r str>,
}

impl UserSettings<'_> {
    /**
     * save new profile settings
     */
    pub fn save(&self, username: &str) -> Result<(), Box<dyn Error>> {
        // save display name
        let user = db::user::UpdateUserBuilder::default()
            .display_name(self.displayname.map(|s| s.to_string()))
            .build()?;
        db::user::update(username.to_string(), user)?;
        Ok(())
    }
}

#[derive(FromForm)]
pub struct ClientSettings<'r> {
    language: &'r str,
}

impl ClientSettings<'_> {
    pub fn save(&self, jar: &CookieJar<'_>) {
        // save language
        set_language_cookie(self.language, jar)
    }
}

pub fn set_language_cookie(value: &str, jar: &CookieJar<'_>) {
    jar.add(Cookie::new("language", value.to_owned()))
}

/**
 * settings for changing password
 */
#[derive(FromForm)]
pub struct PasswordSettings<'r> {
    password: &'r str,
    new_password: &'r str,
    delete: bool,
}

impl PasswordSettings<'_> {
    pub fn change_password(&self, username: &str) -> Result<(), Box<dyn Error>> {
        // errors if the login is not valid
        if !auth::is_valid_login(username, self.password)? {
            return Err(SettingsError::LoginInvalid)?;
        }

        // delete account
        if self.delete {
            db::user::delete(username.into())?;
            return Ok(());
        }

        // dont allow empty passwords
        if self.new_password.is_empty() {
            return Err(SettingsError::NoPassword)?;
        }

        auth::change_password(username.to_string(), self.new_password.to_string())?;
        Ok(())
    }
}
