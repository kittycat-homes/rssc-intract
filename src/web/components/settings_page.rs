use crate::{
    database::user::User,
    web::{components::common::language_picker::*, language::Translation},
};
use sycamore::prelude::*;

use super::common::accent_color::random_color;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let accent_color = random_color(1)[0];
    let possibly_authenticated_settings = match props.user {
        None => view! {cx,
            a (href="/login", class=format!("link_button {}", accent_color)){
                (props.translation.go_to_login_for_more_settings)
            }
            br {}
        },
        // there is an authenticated user,
        // this means we can show all settings
        Some(user) => AuthenticatedSettings(
            cx,
            AuthenticatedSettingsProps {
                user,
                translation: props.translation,
                accent_color,
            },
        ),
    };
    view! {cx,
        div {
            h1 (class=accent_color) { (props.translation.settings_page_heading) }
            LanguageForm(LanguageFormProps {
                translation: props.translation,
                accent_color
            })
            (possibly_authenticated_settings)
            a (href="/my_data", class=format!("link_button {}", accent_color)) {(props.translation.my_data)}
        }
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
    pub accent_color: &'static str,
}

#[derive(Prop)]
struct LanguageFormProps {
    pub translation: Translation,
    pub accent_color: &'static str,
}

#[component]
fn LanguageForm(cx: Scope, props: LanguageFormProps) -> View<SsrNode> {
    view! {cx,
        form (action="/settings/client", method="post") {
            h2 {(props.translation.settings_page_client_heading)}
            LanguagePicker(LanguagePickerProps { translation: props.translation })
            br {}
            input (type="submit",
                   class=format!("link_button {}", props.accent_color),
                   value=(format!("{} 💾", props.translation.save))) {}
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
                input (type="submit", class=format!("link_button {}", props.accent_color), value=(format!("{} 💾", props.translation.save))) {}
            }

            form (action="/settings/password", method="post"){
                h3 {(props.translation.settings_page_password_heading)}
                // new password
                label (for="new_password"){(props.translation.new_password)}
                br {}
                input (type="password", id="new_password", name="new_password") {}
                br {}
                input (type="checkbox", id="delete", name="delete") {}
                label (for="delete") {(props.translation.delete_my_account)}
                br {}
                // old password
                label (for="password"){(props.translation.password)}
                br {}
                input (type="password", id="password", name="password") {}
                input (type="submit",
                       class=format!("link_button {}", props.accent_color),
                       onclick=format!("return confirm('{}')", props.translation.irreversible_changes_warning),
                       value=(format!("{} 💾", props.translation.save))) {
                }
            }
        a (href="/logout", class=format!("link_button {}", props.accent_color)){(props.translation.logout)}
        }
    }
}
