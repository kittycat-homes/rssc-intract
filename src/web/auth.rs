use crate::logic::auth as logic;
use rocket::{
    form::Form,
    http::CookieJar,
    response::{Flash, Redirect},
    Route,
};
use rocket_dyn_templates::{context, Template};

pub fn routes() -> Vec<Route> {
    routes![post_login, login, login_redirect]
}

/**
 * accepts login form
 */
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

/**
 * redirects user to front page if they are already logged in
 */
#[get("/login")]
fn login_redirect(_user: logic::User) -> Redirect {
    Redirect::to("/")
}

/**
 * shows login page if a user is not logged in
 */
#[get("/login", rank = 2)]
fn login() -> Template {
    Template::render("login", context! {})
}
