use crate::database::schema::tags;
use crate::database::*;

#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
    pub username: String,
    pub post_id: String,
}

/// create new tag
pub fn create(mut tag: Tag) -> QueryResult<Tag> {
    let connection = &mut establish_connection();

    tag.tag = tag.tag.to_lowercase();

    diesel::insert_into(tags::table)
        .values(tag)
        .get_result(connection)
}

/// search tags
pub fn search(search: String) -> QueryResult<Vec<Tag>> {
    let connection = &mut establish_connection();

    let pattern = format!("{}%", search);

    tags::table
        .filter(tags::tag.like(pattern))
        .load::<Tag>(connection)
}
