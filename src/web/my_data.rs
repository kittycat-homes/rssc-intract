use rocket::{response::content::RawHtml, Route};

use crate::{
    logic::auth::Session,
    web::{components, language::Translation},
};

pub fn routes() -> Vec<Route> {
    routes![my_data]
}

#[get("/my_data")]
fn my_data(session: Option<Session>, translation: Translation) -> RawHtml<String> {
    let authenticated = session.is_some();
    components::render_page(
        components::Pages::MyData {
            props: components::my_data::Props {
                translation,
                user: session.map(|s| s.user),
            },
        },
        translation,
        authenticated,
    )
}
