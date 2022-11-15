use rocket::response::content::RawHtml;
use sycamore::{builder::prelude::*, prelude::*, render_to_string};

use super::language::Translation;

pub mod login_page;
pub mod profile_page;
pub mod settings_page;

/// renders a page to raw html
pub fn render_page(page: Pages, translation: Translation) -> RawHtml<String> {
    RawHtml(format!(
        "<!DOCTYPE html>{}",
        render_to_string(|cx| App(
            cx,
            AppProps {
                content: page,
                translation
            }
        ))
    ))
}

/**
 * lists avialable pages to render
 * these usually correspond to a route
 */
pub enum Pages {
    /// a users profile, displaying their recent shares etc.
    Profile {
        props: profile_page::Props,
    },
    Settings {
        props: settings_page::Props,
    },
    Login {
        props: login_page::Props,
    },
}

#[derive(Prop)]
struct AppProps {
    content: Pages,
    translation: Translation,
}

#[component]
fn App(cx: Scope, props: AppProps) -> View<SsrNode> {
    html()
        .attr("lang", props.translation.code)
        .c(Head(
            cx,
            match &props.content {
                // pick the appropriate component to render for each page
                Pages::Settings { props } => {
                    format!("settings for @{} | rssc-intract", props.user.username)
                }
                Pages::Profile { props } => format!("{} | rssc-intract", props.user.username),
                Pages::Login { props: _ } => format!("{} rssc-intract", "login"),
            },
        ))
        .c(body().c(div().id("content").c(match props.content {
            Pages::Profile { props } => profile_page::Page(cx, props),
            Pages::Settings { props } => settings_page::Page(cx, props),
            Pages::Login { props } => login_page::Page(cx, props),
        })))
        .view(cx)
}

#[component]
fn Head(cx: Scope, title: String) -> View<SsrNode> {
    view! {cx, head {
        meta (name="viewport", content="width=device-width, initial-scale=1")
        title {(title)}
    }}
}
