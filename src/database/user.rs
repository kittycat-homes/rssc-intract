use crate::database::models::*;
use crate::database::*;

pub fn create_user(mut user: NewUser) -> User {
    use crate::database::schema::users;
    let connection = &mut establish_connection();

    user.username = user.username.to_lowercase();

    diesel::insert_into(users::table)
        .values(user)
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

// check if given session id already exists

// check who session id belongs to

// list of everyone user is following

// list of all the feeds a user is subscribed to
