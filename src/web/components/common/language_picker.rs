use crate::web::language::Translation;
use sycamore::prelude::*;

#[component]
pub fn LanguagePicker(cx: Scope, props: LanguagePickerProps) -> View<SsrNode> {
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
pub struct LanguagePickerProps {
    pub translation: Translation,
}

#[component]
pub fn LanguageSelectionItem(cx: Scope, props: LanguageSelectionItemProps) -> View<SsrNode> {
    view! {cx,
        option (value=(props.code), selected=props.translation.code == props.code) {(props.name)}
    }
}

#[derive(Prop)]
pub struct LanguageSelectionItemProps {
    pub translation: Translation,
    pub code: &'static str,
    pub name: &'static str,
}
