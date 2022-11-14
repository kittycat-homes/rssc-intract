use crate::database::user::User;
use sycamore::prelude::*;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let display_name = match props.user.display_name {
        Some(e) => e,
        None => "".to_string(),
    };

    view! {cx,
        h1 {"settings"}

        h2 {"profile"}
        form (action="/settings/profile", method="post"){
            label (for="displayname") {"display name"}
            input (type="text", id="displayname", name="displayname", value=(display_name)){}
            input (type="submit", value="change") {}
        }

        h2 {"password"}
        form (action="/settings/password", method="post"){
            // old password
            label (for="password"){"old password"}
            input (type="password", id="password", name="password") {}
            br {}
            // new password
            label (for="new_password"){"new password"}
            input (type="password", id="new_password", name="new_password") {}

            input (type="submit", value="change") {}
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub user: User,
}
