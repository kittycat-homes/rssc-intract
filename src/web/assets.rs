use rocket::{fairing::Fairing, route::Route};
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};

static_response_handler! {
    "/css/tailwind.css" => css => "css",
    "/fonts/atkinson_bold.woff2" => atkinson_bold => "atkinson_bold",
    "/fonts/atkinson_regular.woff2" => atkinson_regular => "atkinson_regular",
}

pub fn routes() -> Vec<Route> {
    routes![css, atkinson_regular, atkinson_bold]
}

pub fn fairing() -> impl Fairing {
    static_resources_initializer!(
    "css" => "web/static/css/tailwind.css",
    "atkinson_bold" => "web/static/fonts/atkinson_bold.woff2",
    "atkinson_regular" => "web/static/fonts/atkinson_regular.woff2"
    )
}
