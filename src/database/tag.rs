use crate::database::*;

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
    pub username: String,
    pub post_id: String,
}
