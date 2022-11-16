use crate::database::user::User;
use sycamore::prelude::*;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let display_name = match props.user.display_name {
        Some(e) => e,
        None => "".to_string(),
    };

    let profiletext = format!("@{}", props.user.username);

    view! {cx,
        h1 {"seeettings"}

        h2 {(profiletext)}
        form (action="/settings", method="post"){
            label (for="displayname") {"display name"}
            br {}
            input (type="text", id="displayname", name="displayname", value=(display_name)){}
            br {}
            label {"🌐"}
            input (type="text", id="language", name="language") {}
            input (type="submit", value="change") {}
        }

        h2 {"password"}
        form (action="/settings/password", method="post"){
            // old password
            label (for="password"){"old password"}
            br {}
            input (type="password", id="password", name="password") {}
            br {}
            // new password
            label (for="new_password"){"new password"}
            br {}
            input (type="password", id="new_password", name="new_password") {}
            br {}
            input (type="submit", value="change") {}
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub user: User,
}
