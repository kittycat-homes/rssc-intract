#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::{
    database as db,
    logic::{self, auth::Session},
    web::components::{Pages, ProfilePageProps},
};
use rocket::{
    form::Form,
    http::Status,
    response::{content, Redirect},
    Route,
};
use serde::Serialize;

use super::components::render_page;

mod userpage;

pub fn routes() -> Vec<Route> {
    routes![follow]
}

#[derive(FromForm)]
struct Id<'r> {
    userid: &'r str,
}

#[derive(Serialize)]
struct ProfileViewData {
    user_name: String,
    display_name: Option<String>,
}

#[get("/user/<username>")]
pub fn profile(username: String) -> Result<content::RawHtml<String>, Status> {
    let user = match db::user::get(username.clone()) {
        Ok(u) => u,
        Err(e) => {
            error!("{}", e);
            return Err(Status::NotFound);
        }
    };

    Ok(render_page(Pages::UserPage {
        props: ProfilePageProps { user },
    }))
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

    //TODO check if the person to follow actually exists

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
