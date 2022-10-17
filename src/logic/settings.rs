use crate::database as db;
use crate::logic::auth;
use rocket::FromForm;
use std::error::Error;

#[derive(thiserror::Error, Debug)]
enum SettingsError {
    #[error("login could not be validated")]
    LoginInvalid,
}

/**
 * struct used to update settings
 * settings that should not be changed should be None
 */
#[derive(FromForm)]
pub struct ProfileSettings<'r> {
    displayname: &'r str,
}

impl ProfileSettings<'_> {
    /**
     * save new profile settings
     */
    pub fn save(&self, username: &str) -> Result<(), Box<dyn Error>> {
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
        // errors if the login is not valid
        if !auth::is_valid_login(username, self.password)? {
            return Err(SettingsError::LoginInvalid)?;
        }

        auth::change_password(username.to_string(), self.new_password.to_string())?;
        Ok(())
    }
}
