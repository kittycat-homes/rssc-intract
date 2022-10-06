use crate::logic::auth as logic;

use rocket::{
    form::Form,
    http::CookieJar,
    response::{Flash, Redirect},
    Route,
};

pub fn routes() -> Vec<Route> {
    routes![post_login]
}

#[post("/login", data = "<login>")]
fn post_login(
    jar: &CookieJar<'_>,
    login: Form<logic::Login<'_>>,
) -> Result<Redirect, Flash<Redirect>> {
    match logic::login(jar, login.into_inner()) {
        Ok(_) => Ok(Redirect::to("/")),
        Err(e) => Err(Flash::error(Redirect::to("/login"), e.1)),
    }
}
