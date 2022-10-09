use crate::database::schema::sessionid;
use crate::database::user::User;
use crate::database::*;

/// struct that represents the sessionid table
#[derive(Queryable)]
pub struct SessionID {
    pub id: String,
    pub username: String,
    pub last_active: Option<std::time::SystemTime>,
    pub name: Option<String>,
}

/// struct for inserting new session ids
#[derive(Insertable, Builder)]
#[diesel(table_name = sessionid)]
pub struct NewSessionID {
    pub id: String,
    pub username: String,
    #[builder(default)]
    pub last_active: Option<std::time::SystemTime>,
    #[builder(default)]
    pub name: Option<String>,
}

/// returns sessionID
pub fn get(id: String) -> QueryResult<SessionID> {
    let connection = &mut establish_connection();
    sessionid::table.find(id).first::<SessionID>(connection)
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
pub fn create(id: NewSessionID) -> QueryResult<SessionID> {
    let connection = &mut establish_connection();

    diesel::insert_into(sessionid::table)
        .values(id)
        .get_result(connection)
}
