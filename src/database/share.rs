use crate::database::post::Post;
use crate::database::schema::shares;
use crate::database::*;

/// struct that represents shares
#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = shares)]
pub struct Share {
    pub post_id: String,
    pub username: String,
    #[builder(default)]
    pub user_comment: Option<String>,
    pub time: std::time::SystemTime,
}

/// create new share (WORKS)
pub fn create(share: Share) -> QueryResult<Share> {
    let connection = &mut establish_connection();

    diesel::insert_into(shares::table)
        .values(share)
        .get_result(connection)
}

/// delete share (WORKS)
pub fn delete(post_id: String, username: String) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    diesel::delete(
        shares::table
            .filter(shares::post_id.eq(post_id))
            .filter(shares::username.eq(username)),
    )
    .execute(connection)
}

/// get share from all user's friends (WORKS)
pub fn get_shares_from_friend(username: String) -> QueryResult<Vec<Share>> {
    use crate::database::schema::follows;
    let connection = &mut establish_connection();

    follows::table
        .filter(follows::follower.eq(username))
        .inner_join(shares::table.on(shares::username.eq(follows::followed)))
        .select((
            shares::post_id,
            shares::username,
            shares::user_comment,
            shares::time,
        ))
        .load::<Share>(connection)
}

/// get post from all shares from user's friends (WORKS)
pub fn get_posts_from_friend(username: String) -> QueryResult<Vec<Post>> {
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
        .distinct()
        .load::<Post>(connection)
}

/// get amount of shares from user's friends for a given post (WORKS)
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
