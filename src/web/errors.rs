use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorContext<'r> {
    pub message: &'r str,
}

pub fn render_error(data: ErrorContext) -> Template {
    Template::render("error", data)
}
