use crate::database::post::Post;
use crate::database::schema::tags;
use crate::database::*;

/// struct for representing tags
#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub value: String,
    pub username: String,
    pub post_id: String,
}

/// we need a separate struct for new tags so that postgres can increment the id for us
#[derive(Insertable, Builder, Debug, PartialEq)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub value: String,
    pub username: String,
    pub post_id: String,
}

/// create new tag
pub fn create(mut tag: NewTag) -> QueryResult<Tag> {
    let connection = &mut establish_connection();

    tag.value = tag.value.to_lowercase();

    diesel::insert_into(tags::table)
        .values(tag)
        .get_result(connection)
}

/// search tags
pub fn search(search: String, username: String) -> QueryResult<Vec<Tag>> {
    use crate::database::schema::follows;
    let connection = &mut establish_connection();

    let pattern = format!("%{}%", search);

    follows::table
        .filter(follows::follower.eq(username))
        .inner_join(
            tags::table.on(tags::username
                .eq(follows::followed)
                .or(tags::username.eq(follows::follower))),
        )
        .filter(tags::value.ilike(pattern))
        .select((tags::id, tags::value, tags::username, tags::post_id))
        .load::<Tag>(connection)
}

/// search posts (using what your friends have tagged it with)
pub fn search_posts(search: String, username: String) -> QueryResult<Vec<Post>> {
    use crate::database::schema::{follows, posts};
    let connection = &mut establish_connection();

    follows::table
        .filter(follows::follower.eq(username))
        .inner_join(
            tags::table.on(tags::username
                .eq(follows::followed)
                .or(tags::username.eq(follows::follower))),
        )
        .filter(tags::value.ilike(search))
        .inner_join(posts::table.on(posts::id.eq(tags::post_id)))
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

/// gets all tags attached to post (from friends)
pub fn get_post_tags(post_id: String, username: String) -> QueryResult<Vec<Tag>> {
    use crate::database::schema::follows;
    let connection = &mut establish_connection();

    follows::table
        .filter(follows::follower.eq(username))
        .inner_join(
            tags::table.on(tags::username
                .eq(follows::followed)
                .or(tags::username.eq(follows::follower))),
        )
        .filter(tags::post_id.eq(post_id))
        .select((tags::id, tags::value, tags::username, tags::post_id))
        .distinct()
        .load::<Tag>(connection)
}
