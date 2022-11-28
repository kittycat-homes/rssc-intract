use diesel::{data_types::PgInterval, Queryable};

#[derive(Queryable)]
pub struct Caption {
    pub id: i32,
    pub content: String,
    pub start_time: PgInterval,
    pub end_time: PgInterval,
    pub media: i32,
}
