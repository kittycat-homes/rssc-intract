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
pub struct Settings<'r> {
    displayname: &'r str,
    language: &'r str,
}

impl Settings<'_> {
    /**
     * save new profile settings
     */
    pub fn save(&self, username: &str, jar: &CookieJar<'_>) -> Result<(), Box<dyn Error>> {
        // save language
        jar.add(Cookie::new("language", self.language.to_string()));

        // save display name
        let user = db::user::UpdateUserBuilder::default()
            .display_name(Some(self.displayname.to_string()))
            .build()?;
        db::user::update(username.to_string(), user)?;
        Ok(())
    }
}

/**
 * settings for changing password
 */
#[derive(FromForm)]
pub struct PasswordSettings<'r> {
    password: &'r str,
    new_password: &'r str,
}

impl PasswordSettings<'_> {
    pub fn change_password(&self, username: &str) -> Result<(), Box<dyn Error>> {
        // dont allow empty passwords
        if self.new_password.is_empty() {
            return Err(SettingsError::NoPassword)?;
        }
        // errors if the login is not valid
        if !auth::is_valid_login(username, self.password)? {
            return Err(SettingsError::LoginInvalid)?;
        }

        auth::change_password(username.to_string(), self.new_password.to_string())?;
        Ok(())
    }
}
