// this is fine since we actually expect to return errors fairly often
// and the options page shouldnt be pinged that much anyway
#![allow(clippy::result_large_err)]

use crate::logic;
use crate::web::{components, errors, language::Translation};
use rocket::response::content::RawHtml;
use rocket::{
    form::Form,
    http::CookieJar,
    response::{Flash, Redirect},
    Route,
};
use rocket_dyn_templates::Template;

pub fn routes() -> Vec<Route> {
    routes![post_login, login, login_redirect, logout, login_language]
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

#[derive(FromForm)]
pub struct Language<'r> {
    language: &'r str,
}

impl Language<'_> {
    pub fn save(&self, jar: &CookieJar<'_>) {
        // save language
        logic::settings::set_language_cookie(self.language, jar)
    }
}

#[post("/login/language", data = "<language>")]
fn login_language(jar: &CookieJar<'_>, language: Form<Language>) -> Redirect {
    language.save(jar);
    Redirect::to("/login")
}

/**
 * shows login page if a user is not logged in
 */
#[get("/login", rank = 2)]
fn login(translation: Translation) -> RawHtml<String> {
    components::render_page(
        components::Pages::Login {
            props: components::login_page::Props { translation },
        },
        translation,
    )
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
