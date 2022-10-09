#[derive(Queryable)]
pub struct Follow {
    pub username: String,
    pub following: String,
}
