use crate::database::schema::sessionid;
use crate::database::user::User;
use crate::database::*;
use std::time::SystemTime;

/// struct that represents the sessionid table
#[derive(Insertable, Queryable, Builder)]
#[diesel(table_name = sessionid)]
pub struct SessionID {
    pub id: String,
    pub username: String,
    pub last_active: SystemTime,
    #[builder(default)]
    pub name: Option<String>,
}

/// struct for updating sessionids
#[derive(AsChangeset, Builder)]
#[diesel(table_name = sessionid)]
pub struct UpdateSessionID {
    #[builder(default)]
    pub last_active: Option<SystemTime>,
    #[builder(default)]
    pub name: Option<String>,
}

/// returns sessionID
pub fn get(id: String) -> QueryResult<SessionID> {
    let connection = &mut establish_connection();
    sessionid::table.find(id).first::<SessionID>(connection)
}

/// gets sessionids belonging to user
pub fn get_from_user(username: String) -> QueryResult<Vec<SessionID>> {
    let connection = &mut establish_connection();

    sessionid::table
        .filter(sessionid::username.eq(username))
        .load::<SessionID>(connection)
}

/// check who session id belongs to
pub fn belongs_to(id: String) -> QueryResult<User> {
    use crate::database::schema::users;
    let connection = &mut establish_connection();

    let user = sessionid::table
        .find(id)
        .select(sessionid::username)
        .first::<String>(connection)?;

    users::table.find(user).first::<User>(connection)
}

/// create sessionid
pub fn create(id: SessionID) -> QueryResult<SessionID> {
    let connection = &mut establish_connection();

    diesel::insert_into(sessionid::table)
        .values(id)
        .get_result(connection)
}

/// deletes sessionid
pub fn delete(id: String) -> QueryResult<usize> {
    let connection = &mut establish_connection();

    diesel::delete(sessionid::table.find(id)).execute(connection)
}

/// update sessionid
pub fn update(id: String, sessionid: UpdateSessionID) -> QueryResult<SessionID> {
    let connection = &mut establish_connection();

    diesel::update(sessionid::table.find(id))
        .set(sessionid)
        .get_result(connection)
}

/// update sessionid last_active
pub fn update_time(id: String, time: SystemTime) -> QueryResult<SessionID> {
    let connection = &mut establish_connection();

    diesel::update(sessionid::table.find(id))
        .set(sessionid::last_active.eq(time))
        .get_result(connection)
}
