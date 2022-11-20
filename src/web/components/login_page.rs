use crate::web::{
    components::common::language_picker::{LanguagePicker, LanguagePickerProps},
    language::Translation,
};
use sycamore::prelude::*;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let translation = props.translation;
    let heading = translation.login_page_heading;
    let username = translation.username;
    let password = translation.password;
    let login = translation.login;

    view! {cx,
        h1 {(heading)}
        form (action="/login", method="post") {
            label (for="username") {
                (username)
            }
            br {}
            input (type="text", id="username", name="username") {}
            br {}
            label (for="password") {
                (password)
            }
            br {}
            input (type="password", id="password", name="password") {}
            br {}
            input (type="submit", value=(login))
        }
        div (class="flex flex-row justify-around items-center w-full") {
            a (href="/settings"){
                (format!("{} ⚙", props.translation.settings_page_heading))
            }
            form (action="/login/language", method="post", class="grid justify-center text-center") {
                LanguagePicker(LanguagePickerProps { translation: props.translation })
                input (type="submit", value=(format!("{} 💾", props.translation.save)))
            }
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
}
