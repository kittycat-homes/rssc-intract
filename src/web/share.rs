use rocket::{
    form::Form,
    response::{content::RawHtml, Redirect},
    route::Route,
};

use crate::{
    logic::{auth::Session, share::ShareForm},
    web::{
        components::{self, new_share_page},
        language::Translation,
    },
};

pub fn routes() -> Vec<Route> {
    routes![new, new_redirect, new_post]
}

#[get("/share/new")]
fn new(session: Session, translation: Translation) -> RawHtml<String> {
    components::render_page(
        components::Pages::NewShare {
            props: new_share_page::Props {
                user: session.user,
                translation,
            },
        },
        translation,
    )
}

#[get("/share/new", rank = 2)]
fn new_redirect() -> Redirect {
    Redirect::to("/login")
}

#[post("/share/new", data = "<share>")]
async fn new_post(session: Session, share: Form<ShareForm<'_>>) -> Redirect {
    match share.save(&session.user.username).await {
        Ok(_) => Redirect::to(format!("/users/{}", session.user.username)),
        Err(e) => {
            error!("{}", e);
            Redirect::to("")
        }
    }
}
