use crate::database as db;
use rocket::FromForm;
use std::error::Error;

/**
 * struct used to update settings
 * settings that should not be changed should be None
 */
#[derive(FromForm)]
pub struct ProfileSettings<'r> {
    pub displayname: &'r str,
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
