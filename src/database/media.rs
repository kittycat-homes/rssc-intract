use diesel::Queryable;

#[derive(Queryable)]
pub struct Media {
    pub id: i32,
    pub url: Option<String>,
    pub title: Option<String>,
    pub mime: Option<String>,
    pub description: Option<String>,
    pub post: Option<String>,
}
