use sycamore::prelude::*;

use crate::web::language::Translation;

use super::common::accent_color::random_color;

#[derive(Prop)]
pub struct Props {
    pub url: Option<String>,
    pub translation: Translation,
}

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let color = random_color(1)[0];
    let url_field_value = props.url.unwrap_or("".into());

    view! {cx,
        h1 (class=color) {(props.translation.subscribe)}
        form (action="/subscribe", method="post") {
            label (for="url") {(props.translation.url)}
            br {}
            input (
                id="url",
                class=format!("{} rounded_input", color),
                name="url",
                type="url",
                value=url_field_value,
            ) {}
        }
    }
}
