use crate::database::post::*;
use crate::database::schema::feeds;
use crate::database::*;
use std::time::SystemTime;

/// struct representing feeds, for getting feeds and making new ones
#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = feeds)]
pub struct Feed {
    pub id: String,
    pub url: String,
    #[builder(default)]
    pub title: Option<String>,
    pub last_updated: SystemTime,
}

/// struct for updating feeds. None means no change
#[derive(AsChangeset, Builder)]
#[diesel(table_name = feeds)]
pub struct UpdateFeed {
    #[builder(default)]
    pub title: Option<String>,
    #[builder(default)]
    pub last_updated: Option<SystemTime>,
}

/// create feed from feed struct
pub fn create(feed: Feed) -> QueryResult<Feed> {
    let connection = &mut establish_connection();

    diesel::insert_into(feeds::table)
        .values(feed)
        .get_result(connection)
}

/// get feed struct from feed id
pub fn get(id: String) -> QueryResult<Feed> {
    let connection = &mut establish_connection();

    feeds::table.find(id).first::<Feed>(connection)
}

/// delete feed (why?)
/// returns number of things that were deleted )
pub fn delete(id: String) -> QueryResult<usize> {
    use crate::database::schema::{posts, subscriptions};
    let connection = &mut establish_connection();

    // delete anything that references this feed
    diesel::update(posts::table.filter(posts::feed_id.eq(&id)))
        .set(posts::feed_id.eq::<Option<String>>(None))
        .execute(connection)?;
    diesel::delete(subscriptions::table.filter(subscriptions::feed_id.eq(&id)))
        .execute(connection)?;

    // delete feed
    diesel::delete(feeds::table.find(id)).execute(connection)
}

/// update feed
pub fn update(id: String, feed: UpdateFeed) -> QueryResult<Feed> {
    let connection = &mut establish_connection();

    diesel::update(feeds::table.find(id))
        .set(feed)
        .get_result(connection)
}

/// specifically update time
pub fn update_time(id: String, time: SystemTime) -> QueryResult<Feed> {
    let connection = &mut establish_connection();

    diesel::update(feeds::table.find(id))
        .set(feeds::last_updated.eq(time))
        .get_result(connection)
}

/// get posts from feed
pub fn get_posts(feed_id: String) -> QueryResult<Vec<Post>> {
    use crate::database::schema::posts;

    let connection = &mut establish_connection();

    posts::table
        .filter(posts::feed_id.eq(feed_id))
        .load::<Post>(connection)
}
