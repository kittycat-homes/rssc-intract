use crate::{database::user::User, web::language::Translation};
use sycamore::prelude::*;

pub struct Props {
    pub user: User,
    pub translation: Translation,
}

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    view! {cx,
        form (action="/share/new",  method="post"){
            label (for="url") {"url"}
            br {}
            input (type="url", id="url", name="url") {}
            br {}
            label (for="user_comment") {(props.translation.comment)}
            br {}
            textarea (id="user_comment", name="user_comment") {}
            br {}
            label (for="tags") {(props.translation.tags)}
            br {}
            textarea (id="tags", name="tags") {}
            br {}
            input (type="submit", value=(props.translation.new_share))
        }
    }
}
