use crate::web::{
    components::common::language_picker::{LanguagePicker, LanguagePickerProps},
    language::Translation,
};
use sycamore::prelude::*;

use super::common::accent_color::random_color;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let accent_colors = random_color(2);
    let accent_color = accent_colors[0];
    let accent_color_1 = accent_colors[1];

    let translation = props.translation;
    let heading = translation.login_page_heading;
    let username = translation.username;
    let password = translation.password;
    let login = translation.login;

    view! {cx,
        div (class=format!("md:p-8 p-4 md:m-8 m-4 border-l-4 {}", accent_color)) {
        h1 (class=accent_color) {(heading)}
        form (action="/login", method="post") {
            label (for="username") {
                (username)
            }
            br {}
            input (type="text",
                   id="username",
                   name="username",
                   class=format!("rounded_input {}", accent_color)) {}
            br {}
            label (for="password") {
                (password)
            }
            br {}
            input (type="password",
                   class=format!("rounded_input {}", accent_color),
                   id="password",
                   name="password") {}
            br {}
            input (type="submit",
                   class=format!("link_button {}", accent_color),
                   value=(login))
        }
        }
        form (action="/login/language", method="post") {
            LanguagePicker(LanguagePickerProps {
                translation: props.translation,
                accent_color: accent_color_1,
            })
            br{}
            input (type="submit",
                    class=format!("link_button {}", accent_color_1),
                    value=(format!("{} 💾", props.translation.save)))
            }
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
}
