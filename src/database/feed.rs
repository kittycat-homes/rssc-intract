use crate::database::models::*;
use crate::database::*;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Feed {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub last_updated: Option<std::time::SystemTime>,
}

// get posts from feed

// get feed information
