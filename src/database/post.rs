use crate::database::schema::posts;
use crate::database::*;

#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: String,
    pub url: String,
    #[builder(default)]
    pub title: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub feed_id: Option<String>,
    pub time: std::time::SystemTime,
}

#[derive(AsChangeset, Builder)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    #[builder(default)]
    pub title: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub feed_id: Option<String>,
    #[builder(default)]
    pub time: Option<std::time::SystemTime>,
}

/// create post from post struct (WORKS)
pub fn create(post: Post) -> QueryResult<Post> {
    let connection = &mut establish_connection();

    diesel::insert_into(posts::table)
        .values(post)
        .get_result(connection)
}

/// get post struct from post id (WORKS)
pub fn get(id: String) -> QueryResult<Post> {
    let connection = &mut establish_connection();

    posts::table.find(id).first::<Post>(connection)
}

/// delete post
/// returns number of things that were deleted (WORKS)
pub fn delete(id: String) -> QueryResult<usize> {
    use crate::database::schema::{shares, tags};
    let connection = &mut establish_connection();

    diesel::delete(tags::table.filter(tags::post_id.eq(&id))).execute(connection)?;

    diesel::delete(shares::table.filter(shares::post_id.eq(&id))).execute(connection)?;

    diesel::delete(posts::table.find(id)).execute(connection)
}

/// update post (WORKS)
pub fn update(id: String, post: UpdatePost) -> QueryResult<Post> {
    let connection = &mut establish_connection();

    diesel::update(posts::table.find(id))
        .set(post)
        .get_result(connection)
}
