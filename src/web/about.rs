use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct AboutData {}

#[get("/about")]
pub fn about() -> Template {
    Template::render("about", AboutData {})
}
