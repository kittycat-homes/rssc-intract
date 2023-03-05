use rocket::response::content::RawHtml;
use sycamore::{builder::prelude::*, prelude::*, render_to_string};

use super::language::Translation;

pub mod common;
pub mod delete_profile;
mod footer;
pub mod login_page;
pub mod my_data;
pub mod profile_page;
pub mod settings_page;
pub mod subscribe_page;

/// renders a page to raw html
pub fn render_page(page: Pages, translation: Translation, authenticated: bool) -> RawHtml<String> {
    RawHtml(format!(
        "<!DOCTYPE html>{}",
        render_to_string(|cx| App(
            cx,
            AppProps {
                content: page,
                translation,
                authenticated,
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
    DeleteProfile {
        props: delete_profile::Props,
    },
    Subscribe {
        props: subscribe_page::Props,
    },
    Login {
        props: login_page::Props,
    },
    MyData {
        props: my_data::Props,
    },
}

#[derive(Prop)]
struct AppProps {
    content: Pages,
    translation: Translation,
    authenticated: bool,
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
                    format!("{} | rssc-intract", props.translation.settings_page_heading,)
                }
                Pages::MyData { props } => {
                    format!("{} | rssc-intract", props.translation.my_data)
                }
                Pages::DeleteProfile { props } => {
                    format!("{} | rssc-intract", props.translation.delete_my_account)
                }
                Pages::Profile { props } => format!("{} | rssc-intract", props.user.username),
                Pages::Login { props } => format!("{} | rssc-intract", props.translation.login),
                Pages::Subscribe { props } => {
                    format!("{} | rssc-intract", props.translation.subscribe)
                }
            },
        ))
        .c(link()
            .attr("rel", "stylesheet")
            .attr("href", "/static/css/tailwind.css"))
        .c(body().class("flex h-screen flex-col").c(main()
            .id("content")
            .class("grid place-items-center grow")
            .c(match props.content {
                Pages::MyData { props } => my_data::Page(cx, props),
                Pages::Profile { props } => profile_page::Page(cx, props),
                Pages::Settings { props } => settings_page::Page(cx, props),
                Pages::DeleteProfile { props } => delete_profile::Page(cx, props),
                Pages::Login { props } => login_page::Page(cx, props),
                Pages::Subscribe { props } => subscribe_page::Page(cx, props),
            })))
        .c(footer::Footer(
            cx,
            footer::Props {
                translation: props.translation,
                authenticated: props.authenticated,
            },
        ))
        .view(cx)
}

#[component]
fn Head(cx: Scope, title: String) -> View<SsrNode> {
    view! {cx, head {
        meta (name="viewport", content="width=device-width, initial-scale=1")
        link (rel="preload", href="/static/fonts/atkinson_regular.woff2", as="font", type="font/woff2", crossorigin=true){}
        link (rel="preload", href="/static/fonts/atkinson_bold.woff2", as="font", type="font/woff2", crossorigin=true){}
        title {(title)}
    }}
}
