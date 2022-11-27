use sycamore::prelude::*;

use crate::web::language::Translation;

#[component]
pub fn Footer(cx: Scope, props: Props) -> View<SsrNode> {
    view! {cx,
        footer (class="fixed inset-x-0 bottom-0 bg-slate-50") {
            a (href="/settings", class="link_button") {(props.translation.settings_page_heading)}
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
}
