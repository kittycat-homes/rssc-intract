use crate::database::models::*;
use crate::database::*;
use diesel::prelude::*;

/// creates a user with a username and password
pub fn create_user(username: String, password: String) -> User {
    use crate::database::schema::users;
    let connection = &mut establish_connection();

    let new_user = NewUser {
        username,
        display_name: None,
        pfp: None,
        hash: Some(password),
        salt: Some("peepee".to_string()),
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .get_result(connection)
        .expect("error creating user")
}

pub fn get_user(username: String) -> User {
    use crate::database::schema::users;
    let connection = &mut establish_connection();

    users::table
        .find(username)
        .first::<User>(connection)
        .expect("couldnt load users")
}
