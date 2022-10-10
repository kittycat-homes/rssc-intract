use crate::database::schema::follows;
use crate::database::schema::users;
use crate::database::user::*;
use crate::database::*;

#[derive(Insertable, Queryable)]
#[diesel(table_name = follows)]
pub struct Follow {
    pub follower: String,
    pub followed: String,
}

/// list of everyone user is following (DONE)
pub fn get_follows(username: String) -> QueryResult<Vec<User>> {
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
/// followed is the username of the one being followed (DONE)
pub fn follow(follower: String, followed: String) -> QueryResult<Follow> {
    let connection = &mut establish_connection();

    let follow = Follow { follower, followed };

    diesel::insert_into(follows::table)
        .values(follow)
        .get_result(connection)
}

/// unfollows unfollower from unfollowed
pub fn unfollow(unfollower: String, unfollowed: String) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    diesel::delete(
        follows::table
            .filter(follows::follower.eq(unfollower))
            .filter(follows::followed.eq(unfollowed)),
    )
    .execute(connection)
}
