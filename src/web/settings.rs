// this is fine since we actually expect to return errors fairly often
// and the options page shouldnt be pinged that much anyway
#![allow(clippy::result_large_err)]

use crate::{
    logic::{
        auth::Session,
        settings::{ClientSettings, DeleteAccount, PasswordSettings, UserSettings},
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
        change_settings,
        change_client_settings,
        change_password_settings,
        delete_profile,
        post_delete_profile,
    ]
}

#[get("/settings")]
fn settings(session: Option<Session>, translation: Translation) -> RawHtml<String> {
    let authenticated: bool = session.is_some();
    components::render_page(
        components::Pages::Settings {
            props: components::settings_page::Props {
                user: session.map(|s| s.user),
                translation,
            },
        },
        translation,
        authenticated,
    )
}

#[get("/settings/delete_profile")]
fn delete_profile(session: Session, translation: Translation) -> RawHtml<String> {
    components::render_page(
        components::Pages::DeleteProfile {
            props: components::delete_profile::Props {
                translation,
                user: session.user,
            },
        },
        translation,
        true,
    )
}

/// regular settings that are not password protected, but a user must be logged in for
#[post("/settings/user", data = "<settings>")]
fn change_settings(session: Session, settings: Form<UserSettings<'_>>) -> Redirect {
    // TODO: proper error handling, say if something went wrong
    let _save = settings.save(&session.user.username);
    Redirect::to("/settings")
}

/// settings for the frontend, u can change these without being logged in
#[post("/settings/client", data = "<settings>")]
fn change_client_settings(settings: Form<ClientSettings>, jar: &CookieJar<'_>) -> Redirect {
    settings.into_inner().save(jar);
    Redirect::to("/settings")
}

/// settings where you have to reauthorize
#[post("/settings/password", data = "<settings>")]
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

#[post("/settings/delete_profile", data = "<data>")]
fn post_delete_profile(
    session: Session,
    data: Form<DeleteAccount<'_>>,
) -> Result<Redirect, Redirect> {
    match data.delete(&session.user.username) {
        Ok(_) => Ok(Redirect::to("/")),
        Err(e) => {
            error!("{}", e);
            Err(Redirect::to("/settings"))
        }
    }
}
