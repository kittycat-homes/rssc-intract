use crate::{database::user::User, web::components::wrap_and_render};
use rocket::response::content::RawHtml;
use sycamore::{self, component, reactive::Scope, view, view::View, web::SsrNode};

pub struct ProfilePage {
    pub user: User,
}

impl ProfilePage {
    pub fn render(self) -> RawHtml<String> {}
}

#[component]
fn UserPage(cx: Scope) -> View<SsrNode> {
    view! {cx, }
}
