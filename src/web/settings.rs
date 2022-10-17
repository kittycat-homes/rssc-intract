use crate::logic::auth::Session;
use rocket::{response::Redirect, Route};
use rocket_dyn_templates::{context, Template};

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
    Template::render(
        "settings",
        context! {
            displayname: session.user.display_name,
        },
    )
}

#[get("/settings", rank = 2)]
fn redirect_settings() -> Redirect {
    Redirect::to("/login")
}

#[post("/settings/profile")]
fn change_profile_settings(_session: Session) -> Redirect {
    Redirect::to("/settings")
}

#[post("/settings/password")]
fn change_password_settings(_session: Session) -> Redirect {
    Redirect::to("/settings")
}
