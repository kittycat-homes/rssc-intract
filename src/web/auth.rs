// this is fine since we actually expect to return errors fairly often
// and the options page shouldnt be pinged that much anyway
#![allow(clippy::result_large_err)]

use crate::logic;
use crate::web::{components, errors, language};
use rocket::response::content::RawHtml;
use rocket::{
    form::Form,
    http::CookieJar,
    response::{Flash, Redirect},
    Route,
};
use rocket_dyn_templates::{context, Template};

pub fn routes() -> Vec<Route> {
    routes![post_login, login, login_redirect, logout]
}

/**
 * accepts login form
 */
#[post("/login", data = "<login>")]
fn post_login(
    jar: &CookieJar<'_>,
    login: Form<logic::auth::Login<'_>>,
) -> Result<Redirect, Flash<Redirect>> {
    match logic::auth::login(jar, login.into_inner()) {
        Ok(_) => Ok(Redirect::to("/")),
        Err(_) => Err(Flash::error(Redirect::to("/login"), "failed to log in")),
    }
}

/**
 * redirects user to front page if they are already logged in
 */
#[get("/login")]
fn login_redirect(_session: logic::auth::Session) -> Redirect {
    Redirect::to("/")
}

/**
 * shows login page if a user is not logged in
 */
#[get("/login", rank = 2)]
fn login() -> RawHtml<String> {
    components::render_page(components::Pages::LoginPage {
        props: components::login_page::Props {
            language: language::determine_language(),
        },
    })
}

#[get("/logout")]
fn logout(session: logic::auth::Session, jar: &CookieJar<'_>) -> Result<Redirect, Template> {
    match logic::auth::logout(jar, &session) {
        Ok(_) => Ok(Redirect::to("/login")),
        Err(e) => {
            error!("{}", e);
            Err(errors::render_error(errors::ErrorContext {
            message: "failed to properly log you out! your session cookie should have been deleted, but try manually clearing cookies just to be safe",
        }))
        }
    }
}
