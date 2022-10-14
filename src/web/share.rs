use rocket::{form::Form, response::Redirect, route::Route};
use rocket_dyn_templates::{context, Template};

use crate::logic::{auth::Session, share::ShareForm};

pub fn routes() -> Vec<Route> {
    routes![new, new_redirect, new_post]
}

#[get("/beacon/deploy")]
fn new(_session: Session) -> Template {
    Template::render("new_share", context! {})
}

#[get("/beacon/deploy", rank = 2)]
fn new_redirect() -> Redirect {
    Redirect::to("/login")
}

#[post("/beacon/deploy", data = "<share>")]
async fn new_post(session: Session, share: Form<ShareForm<'_>>) -> Redirect {
    match share.save(&session.user.username).await {
        Ok(_) => Redirect::to(format!("/users/{}", session.user.username)),
        Err(e) => {
            error!("{}", e);
            Redirect::to("")
        }
    }
}
