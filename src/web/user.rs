use rocket_dyn_templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct ProfileViewData {
    userid: String,
}

#[get("/crew/<userid>")]
pub fn profile(userid: String) -> Template {
    Template::render("profileview", &ProfileViewData { userid })
}
