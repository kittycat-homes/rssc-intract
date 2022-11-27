use sycamore::prelude::*;

use crate::web::language::Translation;

use super::common::accent_color::random_color;

#[component]
pub fn Footer(cx: Scope, props: Props) -> View<SsrNode> {
    let accent_color = random_color(1)[0];

    let login: View<SsrNode> = match props.authenticated {
        false => view! {cx,
            a (href="/login", class=format!("link_button {}", accent_color)) {
                (props.translation.login)
            }
        },
        true => view! {cx, },
    };

    view! {cx,
        footer (class="sticky bottom-0 bg-slate-100") {
            nav (class="flex justify-evenly") {
                a (href="/settings", class=format!("link_button {}", accent_color)) {
                    (props.translation.settings_page_heading)}
                (login)

            }
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
    pub authenticated: bool,
}
