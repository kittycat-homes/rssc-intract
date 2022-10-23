use crate::database::post::Post;
use crate::database::schema::shares;
use crate::database::*;
use std::time::SystemTime;

/// struct that represents shares
#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = shares)]
pub struct Share {
    pub post_id: String,
    pub username: String,
    #[builder(default)]
    pub user_comment: Option<String>,
    pub time: SystemTime,
}

/// create new share
pub fn create(share: Share) -> QueryResult<Share> {
    let connection = &mut establish_connection();

    diesel::insert_into(shares::table)
        .values(share)
        .get_result(connection)
}

/// delete share
pub fn delete(post_id: String, username: String) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    diesel::delete(
        shares::table
            .filter(shares::post_id.eq(post_id))
            .filter(shares::username.eq(username)),
    )
    .execute(connection)
}

/// get share from all user's friends
pub fn get_shares_from_friends(username: String) -> QueryResult<Vec<Share>> {
    use crate::database::schema::follows;
    let connection = &mut establish_connection();

    follows::table
        // filters to only show people username is following
        .filter(follows::follower.eq(username))
        // combines shares table and follows table so that all shares with username in common with
        // someone being followed show up
        .inner_join(shares::table.on(shares::username.eq(follows::followed)))
        .select((
            shares::post_id,
            shares::username,
            shares::user_comment,
            shares::time,
        ))
        .load::<Share>(connection)
}

/// get post from all shares from user's friends
pub fn get_posts_from_friends(username: String) -> QueryResult<Vec<Post>> {
    use crate::database::schema::{follows, posts};
    let connection = &mut establish_connection();

    follows::table
        .filter(follows::follower.eq(username))
        .inner_join(shares::table.on(shares::username.eq(follows::followed)))
        .inner_join(posts::table.on(posts::id.eq(shares::post_id)))
        .select((
            posts::id,
            posts::url,
            posts::title,
            posts::description,
            posts::feed_id,
            posts::time,
        ))
        // only distinct elements
        .distinct()
        .load::<Post>(connection)
}

/// gets all posts that a specific user has shared.
pub fn get_shares_from_user(username: String) -> QueryResult<Vec<Post>> {
    use crate::database::schema::posts;
    let connection = &mut establish_connection();
    shares::table
        .filter(shares::username.eq(username))
        .inner_join(posts::table.on(posts::id.eq(shares::post_id)))
        .select((
            posts::id,
            posts::url,
            posts::title,
            posts::description,
            posts::feed_id,
            posts::time,
        ))
        .load::<Post>(connection)
}

/// get amount of shares from user's friends for a given post
pub fn get_amount_shares(post_id: String, username: String) -> QueryResult<i64> {
    use crate::database::schema::follows;
    let connection = &mut establish_connection();

    follows::table
        .filter(follows::follower.eq(username))
        .inner_join(shares::table.on(shares::username.eq(follows::followed)))
        .filter(shares::post_id.eq(post_id))
        .count()
        .get_result(connection)
}
