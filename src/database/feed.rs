use crate::database::post::*;
use crate::database::schema::feeds;
use crate::database::*;

#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = feeds)]
pub struct Feed {
    pub id: String,
    pub url: String,
    #[builder(default)]
    pub title: Option<String>,
    #[builder(default)]
    pub last_updated: Option<std::time::SystemTime>,
}

#[derive(AsChangeset, Builder)]
#[diesel(table_name = feeds)]
pub struct UpdateFeed {
    #[builder(default)]
    pub title: Option<String>,
    #[builder(default)]
    pub last_updated: Option<std::time::SystemTime>,
}

/// create feed from feed struct (WORKS)
pub fn create(feed: Feed) -> QueryResult<Feed> {
    let connection = &mut establish_connection();

    diesel::insert_into(feeds::table)
        .values(feed)
        .get_result(connection)
}

/// get feed struct from feed id (WORKS)
pub fn get(id: String) -> QueryResult<Feed> {
    let connection = &mut establish_connection();

    feeds::table.find(id).first::<Feed>(connection)
}

/// delete feed (why?)
/// returns number of things that were deleted (WORKS)
pub fn delete(id: String) -> QueryResult<usize> {
    use crate::database::schema::{posts, subscriptions};
    let connection = &mut establish_connection();

    diesel::delete(posts::table.filter(posts::feed_id.eq(&id))).execute(connection)?;

    diesel::delete(subscriptions::table.filter(subscriptions::feed_id.eq(&id)))
        .execute(connection)?;

    diesel::delete(feeds::table.find(id)).execute(connection)
}

/// update feed (WORKS)
pub fn update(id: String, feed: UpdateFeed) -> QueryResult<Feed> {
    let connection = &mut establish_connection();

    diesel::update(feeds::table.find(id))
        .set(feed)
        .get_result(connection)
}

/// get posts from feed (WORKS)
pub fn get_posts(feed_id: String) -> QueryResult<Vec<Post>> {
    use crate::database::schema::posts;

    let connection = &mut establish_connection();

    posts::table
        .filter(posts::feed_id.eq(feed_id))
        .load::<Post>(connection)
}
