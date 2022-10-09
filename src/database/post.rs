#[derive(Queryable)]
pub struct Post {
    pub id: String,
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub feed_id: Option<String>,
    pub time: std::time::SystemTime,
}
