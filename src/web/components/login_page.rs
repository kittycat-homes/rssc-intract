use crate::web::language::{Language, Translatable};
use sycamore::prelude::*;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let translation = props.translation();
    let heading = translation.login_page_heading;
    let username = translation.username;
    let password = translation.password;

    view! {cx,
        h1 {(heading)}
        form (action="/login", method="post") {
            label (for="username") {
                (username)
            }
            br {}
            input (id="username", name="username") {}
            br {}
            label (for="password") {
                (password)
            }
            br {}
            input (id="password", name="password") {}
            br {}
            input (type="submit", value="login")
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub language: Language,
}

impl Translatable for Props {
    fn language(&self) -> Language {
        self.language
    }
}
