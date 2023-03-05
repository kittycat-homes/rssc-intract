use reqwest::Url;
use rocket::{
    response::{content::RawHtml, Redirect},
    Route,
};

use crate::{
    logic::auth::Session,
    web::{components, language::Translation},
};

pub fn routes() -> Vec<Route> {
    routes![subscribe, add_subscription]
}

/**
* display a page that allows users to subscribe to an rss feed by entering its url
*/
#[get("/subscribe?<url>")]
fn subscribe(_session: Session, translation: Translation, url: Option<String>) -> RawHtml<String> {
    // render component
    components::render_page(
        components::Pages::Subscribe {
            props: components::subscribe_page::Props { translation, url },
        },
        translation,
        // you can only view this page when logged in,
        // so this is possible to hardcode
        true,
    )
}

#[post("/subscribe")]
fn add_subscription() -> Redirect {
    Redirect::to("/")
}
