// this is fine since we actually expect to return errors fairly often
// and the options page shouldnt be pinged that much anyway
#![allow(clippy::result_large_err)]

use crate::{
    logic::{
        auth::Session,
        settings::{PasswordSettings, Settings},
    },
    web::{components, language::Translation},
};

use rocket::{
    form::Form,
    http::CookieJar,
    response::{content::RawHtml, Redirect},
    Route,
};

pub fn routes() -> Vec<Route> {
    routes![
        settings,
        redirect_settings,
        change_settings,
        change_password_settings
    ]
}

#[get("/settings")]
fn settings(session: Session, translation: Translation) -> RawHtml<String> {
    components::render_page(
        components::Pages::Settings {
            props: components::settings_page::Props { user: session.user },
        },
        translation,
    )
}

#[get("/settings", rank = 2)]
fn redirect_settings() -> Redirect {
    Redirect::to("/login")
}

#[post("/settings", data = "<settings>")]
/// regular settings that are not password protected
fn change_settings(
    session: Session,
    settings: Form<Settings<'_>>,
    jar: &CookieJar<'_>,
) -> Redirect {
    // TODO: proper error handling, say if something went wrong
    let _save = settings.save(&session.user.username, jar);
    Redirect::to("/settings")
}

#[post("/settings/password", data = "<settings>")]
/// settings where you have to reauthorize
fn change_password_settings(
    session: Session,
    settings: Form<PasswordSettings<'_>>,
) -> Result<Redirect, Redirect> {
    let save = settings.change_password(&session.user.username);
    match save {
        Ok(_) => Ok(Redirect::to("/login")),
        Err(e) => {
            error!("{}", e);
            Err(Redirect::to("/settings"))
        }
    }
}
