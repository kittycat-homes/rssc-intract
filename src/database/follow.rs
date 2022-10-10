use crate::database::schema::follows;
use crate::database::*;

#[derive(Insertable, Queryable)]
#[diesel(table_name = follows)]
pub struct Follow {
    pub follower: String,
    pub followed: String,
}
