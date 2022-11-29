use crate::database::schema::users;
use crate::database::*;

/// struct that represents the users table
#[derive(Debug, Insertable, Queryable, Builder, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub username: String,
    #[builder(default)]
    pub display_name: Option<String>,
    #[builder(default)]
    pub hash: Option<String>,
}

/// struct used for updating user. None values means no change
/// this struct is separate from User because we don't want to be able to update the primary key.
#[derive(AsChangeset, Builder)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    #[builder(default)]
    pub display_name: Option<String>,
    #[builder(default)]
    pub hash: Option<String>,
}

/// create user from User struct
pub fn create(mut user: User) -> QueryResult<User> {
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

pub fn get_all() -> QueryResult<Vec<User>> {
    let connection = &mut establish_connection();

    users::table.load::<User>(connection)
}

pub fn get_local() -> QueryResult<Vec<User>> {
    let connection = &mut establish_connection();

    users::table
        .filter(users::hash.is_not_null())
        .load::<User>(connection)
}

/// updates user using UpdateUser struct
pub fn update(username: String, user: UpdateUser) -> QueryResult<User> {
    let connection = &mut establish_connection();

    diesel::update(users::table.find(username))
        .set(user)
        .get_result(connection)
}

/// deletes user
pub fn delete(username: String) -> QueryResult<usize> {
    use crate::database::schema::{sessionid, subscriptions};
    let connection = &mut establish_connection();

    // clear username foreign key references
    diesel::delete(subscriptions::table.filter(subscriptions::username.eq(&username)))
        .execute(connection)?;
    diesel::delete(sessionid::table.filter(sessionid::username.eq(&username)))
        .execute(connection)?;

    // finally delete user
    diesel::delete(users::table.find(username)).execute(connection)
}
