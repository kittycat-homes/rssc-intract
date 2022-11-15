use crate::database::user::User;
use sycamore::{component, reactive::Scope, view, view::View, web::SsrNode, Prop};

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    // if it exists, show display name
    // otherwise show username
    let heading = format!(
        "{}'s profile",
        match props.user.display_name {
            Some(display_name) => display_name,
            None => props.user.username,
        }
    );

    view! {cx, h1 {(heading)} }
}

#[derive(Prop)]
pub struct Props {
    pub user: User,
}
