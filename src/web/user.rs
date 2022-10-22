use crate::{
    database as db,
    logic::{self, auth::Session},
};
use rocket::{form::Form, http::Status, response::Redirect, Route};
use rocket_dyn_templates::Template;
use serde::Serialize;

pub fn routes() -> Vec<Route> {
    routes![follow]
}

#[derive(FromForm)]
struct Id<'r> {
    userid: &'r str,
}

#[derive(Serialize)]
struct ProfileViewData {
    userid: String,
}

#[get("/user/<username>")]
pub fn profile(username: String) -> Template {
    Template::render("profileview", &ProfileViewData { userid: username })
}

/// lets you follow a user with format user@url.example
#[post("/user/follow", data = "<data>")]
fn follow(data: Form<Id>, session: Session) -> Result<Redirect, Status> {
    let id = match logic::user::Userid::parse(data.userid.to_string()) {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return Err(Status::BadRequest);
        }
    };

    match db::follow::follow(session.user.username.clone(), id.unparse()) {
        Ok(o) => o,
        Err(e) => {
            error!("{}", e);
            return Err(Status::InternalServerError);
        }
    };

    info!("following {} on {:?}", id.name, id.url);
    Ok(Redirect::to(format!(
        "/user/{}/following",
        session.user.username
    )))
}
