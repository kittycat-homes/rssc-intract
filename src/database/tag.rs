use crate::database::post::Post;
use crate::database::schema::tags;
use crate::database::*;

#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = tags)]
pub struct Tag {
    #[builder(setter(skip))]
    pub id: i32,
    pub tag: String,
    pub username: String,
    pub post_id: String,
}

/// create new tag (WORKS)
pub fn create(mut tag: Tag) -> QueryResult<Tag> {
    let connection = &mut establish_connection();

    tag.tag = tag.tag.to_lowercase();

    diesel::insert_into(tags::table)
        .values(tag)
        .get_result(connection)
}

/// search tags (WORKS)
pub fn search(search: String) -> QueryResult<Vec<Tag>> {
    let connection = &mut establish_connection();

    let pattern = format!("{}%", search);

    tags::table
        .filter(tags::tag.ilike(pattern))
        .load::<Tag>(connection)
}

/// search posts (TODO: make it only search thru tags from friends)
pub fn search_posts(search: String) -> QueryResult<Vec<Post>> {
    use crate::database::schema::posts;
    let connection = &mut establish_connection();

    tags::table
        .filter(tags::tag.ilike(search))
        .inner_join(posts::table.on(posts::id.eq(tags::post_id)))
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
