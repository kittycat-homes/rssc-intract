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

/// get all shares from user's friends
pub fn get_friend_shares(username: String) -> QueryResult<Vec<Post>> {
    use crate::database::schema::{posts, users};
    let connection = &mut establish_connection();

    users::table
        .inner_join(shares::table.on(shares::username.eq(users::username)))
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
pub fn get_amount_shares(post_id: String) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    shares::table
        .filter(shares::post_id.eq(post_id))
        .execute(connection)
}
