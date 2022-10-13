use rocket::{form::Form, response::Redirect, route::Route};
use rocket_dyn_templates::{context, Template};

use crate::logic::{auth::Session, share::ShareForm};

pub fn routes() -> Vec<Route> {
    routes![new, new_redirect, new_post]
}

#[get("/share/new")]
fn new(_session: Session) -> Template {
    Template::render("new_share", context! {})
}

#[get("/share/new", rank = 2)]
fn new_redirect() -> Redirect {
    Redirect::to("/login")
}

#[post("/share/new", data = "<share>")]
fn new_post(session: Session, share: Form<ShareForm<'_>>) {
    trace!("new share {:?}", share);
    let _s = share.into_share(&session.user.username);
}
