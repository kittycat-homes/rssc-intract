use rocket::response::content::RawHtml;
use sycamore::reactive::Scope;
use sycamore::view::View;
use sycamore::web::SsrNode;
use sycamore::{builder::prelude::*, component, render_to_string, view};

pub mod profile_page;
pub mod settings_page;

/// renders a page to raw html
pub fn render_page(page: Pages) -> RawHtml<String> {
    RawHtml(format!(
        "<!DOCTYPE html>{}",
        render_to_string(|cx| App(cx, page))
    ))
}

/**
 * lists avialable pages to render
 * these usually correspond to a route
 */
pub enum Pages {
    /// a users profile, displaying their recent shares etc.
    ProfilePage {
        props: profile_page::Props,
    },
    SettingsPage {
        props: settings_page::Props,
    },
}

#[component]
fn App(cx: Scope, content: Pages) -> View<SsrNode> {
    html()
        .attr("lang", "en")
        .c(Head(
            cx,
            match &content {
                // pick the appropriate component to render for each page
                Pages::SettingsPage { props } => {
                    format!("{} settings | rssc-intract", props.user.username)
                }
                Pages::ProfilePage { props } => format!("{} | rssc-intract", props.user.username),
            },
        ))
        .c(body().c(div().id("content").c(match content {
            Pages::ProfilePage { props } => profile_page::Page(cx, props),
            Pages::SettingsPage { props } => settings_page::Page(cx, props),
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
