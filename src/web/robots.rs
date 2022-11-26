use crate::CONF;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![robots]
}

#[get("/robots.txt")]
fn robots() -> &'static str {
    &CONF.privacy.robots_txt
}
