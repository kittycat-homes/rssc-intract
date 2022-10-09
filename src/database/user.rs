use crate::database::schema::users;
use crate::database::*;

/// struct that represents the users table
#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub display_name: Option<String>,
    pub hash: Option<String>,
    pub salt: Option<String>,
}

/// struct for inserting new
#[derive(Insertable, Builder)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    #[builder(default)]
    pub display_name: Option<String>,
    #[builder(default)]
    pub hash: Option<String>,
    #[builder(default)]
    pub salt: Option<String>,
}

/// create user from NewUser struct
pub fn create(mut user: NewUser) -> QueryResult<User> {
    let connection = &mut establish_connection();

    user.username = user.username.to_lowercase();

    diesel::insert_into(users::table)
        .values(user)
        .get_result(connection)
}

/// return user struct
pub fn get(username: String) -> QueryResult<User> {
    let connection = &mut establish_connection();

    users::table.find(username).first::<User>(connection)
}

// list of everyone user is following

// list of all the feeds a user is subscribed to
