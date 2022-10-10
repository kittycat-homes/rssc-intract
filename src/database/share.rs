use crate::database::*;

#[derive(Queryable)]
pub struct Share {
    pub post_id: String,
    pub username: String,
    pub user_comment: Option<String>,
    pub time: std::time::SystemTime,
}
