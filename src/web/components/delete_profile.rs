use super::common::accent_color::random_color;
use crate::{database::user::User, web::language::Translation};
use sycamore::prelude::*;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let accent_color = random_color(1)[0];
    view! {cx,
        h1 (class = accent_color) { (props.translation.delete_my_account) }
        form (action="/settings/delete_profile", method="post") {
            label (for="username") { (props.translation.username) }
            br {}
            input (type="text", id="username", name="username", class=format!("rounded_input {}", accent_color)) {}
            br {}
            label (for="username") { (props.translation.password) }
            br {}
            input (type="password", id="password", name="password", class=format!("rounded_input {}", accent_color)) {}
            br {}
            input (type="submit",
                class=format!("pt-1 link_button {}", accent_color),
                value=(format!("{} 💾", props.translation.delete_my_account))) {}
        }

    }
}

#[derive(Prop)]
pub struct Props {
    pub user: User,
    pub translation: Translation,
}
