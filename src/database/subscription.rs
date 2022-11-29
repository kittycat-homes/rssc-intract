use crate::database::feed::*;
use crate::database::schema::subscriptions;
use crate::database::*;

/// struct that represents subscriptions
#[derive(Insertable, Queryable)]
#[diesel(table_name = subscriptions)]
pub struct Subscription {
    pub feed_id: String,
    pub username: String,
}

/// list of all the feeds a user is subscribed to
pub fn get_subscriptions(username: String) -> QueryResult<Vec<Feed>> {
    use crate::database::schema::feeds;
    let connection = &mut establish_connection();
    feeds::table
        .inner_join(subscriptions::table.on(subscriptions::feed_id.eq(feeds::id)))
        .filter(subscriptions::username.eq(username))
        .select((
            feeds::id,
            feeds::url,
            feeds::title,
            feeds::last_updated,
            feeds::description,
            feeds::language,
        ))
        .load::<Feed>(connection)
}

/// subscribes user to feed
pub fn subscribe(username: String, feed_id: String) -> QueryResult<Subscription> {
    let connection = &mut establish_connection();

    let subscription = Subscription { feed_id, username };

    diesel::insert_into(subscriptions::table)
        .values(subscription)
        .get_result(connection)
}

/// unsubscribes user from feed
pub fn unsubscribe(username: String, feed_id: String) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    diesel::delete(
        subscriptions::table
            .filter(subscriptions::username.eq(username))
            .filter(subscriptions::feed_id.eq(feed_id)),
    )
    .execute(connection)
}
