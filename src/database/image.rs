use diesel::Queryable;

#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub uri: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<i32>,
    pub feed: Option<String>,
}
