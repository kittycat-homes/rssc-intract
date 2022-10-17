use crate::logic::{
    auth::Session,
    settings::{PasswordSettings, ProfileSettings},
};
use rocket::{form::Form, outcome::Outcome, response::Redirect, Route};
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
fn settings(session: Session) -> Template {
    show(session, Ok(()))
}

#[get("/settings", rank = 2)]
fn redirect_settings() -> Redirect {
    Redirect::to("/login")
}

#[post("/settings/profile", data = "<settings>")]
fn change_profile_settings(session: Session, settings: Form<ProfileSettings<'_>>) -> Template {
    let save = settings.save(&session.user.username);
    show(session, save)
}

#[post("/settings/password", data = "<settings>")]
fn change_password_settings(
    session: Session,
    settings: Form<PasswordSettings<'_>>,
) -> Result<Redirect, Template> {
    let save = settings.change_password(&session.user.username);
    match save {
        Ok(_) => Ok(Redirect::to("/login")),
        Err(_) => Err(show(session, save)),
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
