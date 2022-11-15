use crate::web::language::Translation;
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
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
}
