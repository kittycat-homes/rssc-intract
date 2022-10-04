use rocket::http::Status;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/**
data used to log a user in with password and userid
**/
#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct UserAuthData {
    pub userid: String,
    pub pass: String,
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

/**
*initial authentication with username and password
*TODO returns encoded token if credentials are true
*/
pub async fn authenticate_user(data: UserAuthData) -> Result<String, (Status, &'static str)> {
    if !get_valid_users().contains(&data) {
        return Err((
            Status::Unauthorized,
            "failed to authenticate user with these credentials",
        ));
    }
    Ok("wow".to_string())
}

/**
TODO
authentication with webtoken
**/
pub fn authenticate_token() {}
