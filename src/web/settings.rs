// this is fine since we actually expect to return errors fairly often
// and the options page shouldnt be pinged that much anyway
#![allow(clippy::result_large_err)]

use crate::{
    logic::{
        auth::Session,
        settings::{PasswordSettings, ProfileSettings},
    },
    web::components,
};
use rocket::{
    form::Form,
    response::{content::RawHtml, Redirect},
    Route,
};
use rocket_dyn_templates::{context, Template};
use std::error::Error;

pub fn routes() -> Vec<Route> {
    routes![
        settings,
        redirect_settings,
        change_profile_settings,
        change_password_settings
    ]
}

#[get("/settings")]
fn settings(session: Session) -> RawHtml<String> {
    components::render_page(components::Pages::SettingsPage {
        props: components::settings_page::Props { user: session.user },
    })
}

#[get("/settings", rank = 2)]
fn redirect_settings() -> Redirect {
    Redirect::to("/login")
}

#[post("/settings/profile", data = "<settings>")]
fn change_profile_settings(session: Session, settings: Form<ProfileSettings<'_>>) -> Redirect {
    let _save = settings.save(&session.user.username);
    Redirect::to("/settings")
}

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

fn show(session: Session, r: Result<(), Box<dyn Error>>) -> Template {
    let template = render_template(session);
    match r {
        Ok(_) => template,
        Err(e) => {
            error!("{}", e);
            template
        }
    }
}

fn render_template(session: Session) -> Template {
    Template::render(
        "settings",
        context! {
            displayname: session.user.display_name,
        },
    )
}
