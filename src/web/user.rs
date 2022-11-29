#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::{
    database as db,
    logic::auth::Session,
    web::{
        components::{profile_page, Pages},
        language::Translation,
    },
};
use rocket::{http::Status, response::content, Route};
use serde::Serialize;

use super::components::render_page;

pub fn routes() -> Vec<Route> {
    routes![profile]
}

#[derive(Serialize)]
struct ProfileViewData {
    user_name: String,
    display_name: Option<String>,
}

#[get("/user/<username>")]
pub fn profile(
    username: String,
    translation: Translation,
    session: Option<Session>,
) -> Result<content::RawHtml<String>, Status> {
    let user = match db::user::get(username) {
        Ok(u) => u,
        Err(e) => {
            error!("{}", e);
            return Err(Status::NotFound);
        }
    };

    Ok(render_page(
        Pages::Profile {
            props: profile_page::Props { user },
        },
        translation,
        session.is_some(),
    ))
}
