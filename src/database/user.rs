use crate::database::feed::*;
use crate::database::follow::*;
use crate::database::schema::users;
use crate::database::*;

/// struct that represents the users table
#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = users)]
pub struct User {
    pub username: String,
    #[builder(default)]
    pub display_name: Option<String>,
    #[builder(default)]
    pub hash: Option<String>,
    #[builder(default)]
    pub salt: Option<String>,
}

/// create user from NewUser struct
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

/// list of everyone user is following
pub fn get_follows(username: String) -> QueryResult<Vec<User>> {
    use crate::database::schema::follows;
    let connection = &mut establish_connection();

    users::table
        .inner_join(follows::table.on(follows::followed.eq(users::username)))
        .filter(follows::follower.eq(username))
        .select((
            users::username,
            users::display_name,
            users::hash,
            users::salt,
        ))
        .load::<User>(connection)
}

/// follow user.  
/// follower is the username of the one following
/// followed is the username of the one being followed
pub fn follow(follower: String, followed: String) -> QueryResult<Follow> {
    use crate::database::schema::follows;
    let connection = &mut establish_connection();

    let follow = Follow { follower, followed };

    diesel::insert_into(follows::table)
        .values(follow)
        .get_result(connection)
}

/// list of all the feeds a user is subscribed to
pub fn get_feeds(username: String) -> QueryResult<Vec<Feed>> {
    use crate::database::schema::{feeds, subscriptions};
    let connection = &mut establish_connection();
    feeds::table
        .inner_join(subscriptions::table.on(subscriptions::feed_id.eq(feeds::id)))
        .filter(subscriptions::username.eq(username))
        .select((feeds::id, feeds::url, feeds::title, feeds::last_updated))
        .load::<Feed>(connection)
}
