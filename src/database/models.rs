use crate::database::schema::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Feed {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Queryable)]
pub struct Follow {
    pub username: String,
    pub following: String,
}

#[derive(Queryable)]
pub struct Post {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub feed_url: Option<String>,
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
    pub post_url: String,
    pub username: String,
}

#[derive(Queryable)]
pub struct Subscription {
    pub feed_url: String,
    pub username: String,
}

#[derive(Queryable)]
pub struct User {
    pub username: String,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub hash: Option<String>,
    pub salt: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub display_name: Option<String>,
    pub pfp: Option<String>,
    pub hash: Option<String>,
    pub salt: Option<String>,
}
