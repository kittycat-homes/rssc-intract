use crate::database::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Feed {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub last_updated: Option<std::time::SystemTime>,
}

#[derive(Queryable)]
pub struct Follow {
    pub username: String,
    pub following: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub feed_id: Option<String>,
    pub time: std::time::SystemTime,
}

#[derive(Queryable)]
pub struct SessionID {
    pub id: String,
    pub username: String,
    pub last_active: Option<std::time::SystemTime>,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct Share {
    pub post_id: String,
    pub username: String,
    pub user_comment: Option<String>,
    pub time: std::time::SystemTime,
}

#[derive(Queryable)]
pub struct Subscription {
    pub feed_id: String,
    pub username: String,
}

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub tag: String,
    pub username: String,
    pub post_id: String,
}

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub display_name: Option<String>,
    pub hash: Option<String>,
    pub salt: Option<String>,
}

#[derive(Insertable, Builder)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    #[builder(default)]
    pub display_name: Option<String>,
    #[builder(default)]
    pub hash: Option<String>,
    #[builder(default)]
    pub salt: Option<String>,
}
