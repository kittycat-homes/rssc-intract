use crate::{database::user::User, web::language::Translation};
use sycamore::prelude::*;

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let display_name = match props.user.display_name {
        Some(e) => e,
        None => "".to_string(),
    };

    view! {cx,
        h1 {(props.translation.settings_page_heading)}

        form (action="/settings", method="post"){
            h2 {(props.translation.settings_page_profile_heading)}
            label (for="displayname") {(props.translation.display_name)}
            br {}
            input (type="text", id="displayname", name="displayname", value=(display_name)){}
            br {}
            h2 {(props.translation.settings_page_client_heading)}
            LanguagePicker(LanguagePickerProps { translation: props.translation })
            br {}
            input (type="submit", value=(format!("{} 💾",props.translation.save))) {}
        }

        h2 {(props.translation.password)}
        form (action="/settings/password", method="post"){
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
    }
}

#[derive(Prop)]
pub struct Props {
    pub user: User,
    pub translation: Translation,
}

#[component]
fn LanguagePicker(cx: Scope, props: LanguagePickerProps) -> View<SsrNode> {
    view! {cx,
        label (for="language") {(format!("{} 🌐", props.translation.language))}
        br {}
        select (type="text", id="language", name="language") {
            // it's probably smart to order these alphabetically when adding more
            LanguageSelectionItem(LanguageSelectionItemProps {
                translation: props.translation, code: "de", name: "deutsch 🇩🇪"
            })
            LanguageSelectionItem(LanguageSelectionItemProps {
                translation: props.translation, code: "en", name: "english 🇺🇸"
            })
        }
    }
}

#[derive(Prop)]
struct LanguagePickerProps {
    translation: Translation,
}

#[component]
fn LanguageSelectionItem(cx: Scope, props: LanguageSelectionItemProps) -> View<SsrNode> {
    view! {cx,
        option (value=(props.code), selected=props.translation.code == props.code) {(props.name)}
    }
}

#[derive(Prop)]
struct LanguageSelectionItemProps {
    pub translation: Translation,
    pub code: &'static str,
    pub name: &'static str,
}
