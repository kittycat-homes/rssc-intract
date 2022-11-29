use diesel::Queryable;

#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub href: Option<String>,
    pub title: Option<String>,
    pub feed: Option<String>,
    pub post: Option<String>,
}
