use serde::{Deserialize, Serialize};

/**
sets permission for what a client is allowed to do
will be serialized to and from a String
for example Read would become "Read"
**/
#[derive(Deserialize, Serialize)]
pub enum Scope {
    /// read userdata
    Read,
    /// change userdata
    Write,
}

/**
data used to log a user in with password and userid
**/
#[derive(Deserialize, Serialize)]
pub struct UserAuthData {
    pub userid: String,
    pub pass: String,
    pub scopes: Vec<Scope>,
}

/**
TODO
initial authentication with username and password
**/
pub fn authenticate_user(data: UserAuthData) {}

/**
TODO
authentication with webtoken
**/
pub fn authenticate_token() {}
