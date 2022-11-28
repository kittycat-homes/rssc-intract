use diesel::Queryable;

#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub term: String,
    pub label: Option<String>,
    pub feed: Option<String>,
    pub post: Option<String>,
}
