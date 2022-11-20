use crate::{
    database::user::User,
    web::{components::common::language_picker::*, language::Translation},
};
use sycamore::prelude::*;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let possibly_authenticated_settings = match props.user {
        None => view! {cx,
            a (href="/login"){(props.translation.go_to_login_for_more_settings)}
        },
        // there is an authenticated user,
        // this means we can show all settings
        Some(user) => AuthenticatedSettings(
            cx,
            AuthenticatedSettingsProps {
                user,
                translation: props.translation,
            },
        ),
    };
    view! {cx,
        h1 {
                (props.translation.settings_page_heading)
        }
        LanguageForm(props.translation)
        (possibly_authenticated_settings)
    }
}

#[derive(Prop)]
pub struct Props {
    // if there is no user, that means u can only change client settings
    pub user: Option<User>,
    pub translation: Translation,
}

#[derive(Prop)]
struct AuthenticatedSettingsProps {
    pub user: User,
    pub translation: Translation,
}

#[component]
fn LanguageForm(cx: Scope, translation: Translation) -> View<SsrNode> {
    view! {cx,
        form (action="/settings/client", method="post") {
            h2 {(translation.settings_page_client_heading)}
            LanguagePicker(LanguagePickerProps { translation })
            br {}
            input (type="submit", value=(format!("{} 💾", translation.save))) {}
        }
    }
}

#[component]
fn AuthenticatedSettings(cx: Scope, props: AuthenticatedSettingsProps) -> View<SsrNode> {
    let display_name = match props.user.display_name {
        Some(e) => e,
        None => "".to_string(),
    };

    view! {cx,
        div {
        h2 {(props.translation.user)}
        form (action="/settings/user", method="post"){
            h3 {(props.translation.settings_page_profile_heading)}
            label (for="displayname") {(props.translation.display_name)}
            br {}
            input (type="text", id="displayname", name="displayname", value=(display_name)){}
            br {}
            input (type="submit", value=(format!("{} 💾", props.translation.save))) {}
        }

        form (action="/settings/password", method="post"){
            h3 {(props.translation.settings_page_password_heading)}
            // old password
            label (for="password"){(props.translation.password)}
            br {}
            input (type="password", id="password", name="password") {}
            br {}
            // new password
            label (for="new_password"){(props.translation.new_password)}
            br {}
            input (type="password", id="new_password", name="new_password") {}
            br {}
            input (type="submit", value=(format!("{} 💾", props.translation.save))) {}
        }
    a (href="/logout"){(props.translation.logout)}
    }
    }
}
