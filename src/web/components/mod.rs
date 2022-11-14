use rocket::response::content::RawHtml;
use sycamore::reactive::Scope;
use sycamore::view::View;
use sycamore::web::SsrNode;
use sycamore::{builder::prelude::*, component, render_to_string, view, Prop};

use crate::database::user::User;

/// renders a page to raw html
pub fn render_page(page: Pages) -> RawHtml<String> {
    RawHtml(format!(
        "<!DOCTYPE html>{}",
        render_to_string(|cx| App(cx, page))
    ))
}

#[derive(Prop)]
pub struct ProfilePageProps {
    pub user: User,
}

/**
 * lists avialable pages to render
 * these usually correspond to a route
 */
pub enum Pages {
    ProfilePage { props: ProfilePageProps },
}

#[component]
fn App(cx: Scope, content: Pages) -> View<SsrNode> {
    html()
        .attr("lang", "en")
        .c(Head(
            cx,
            match &content {
                // pick the appropriate component to render for each page
                Pages::ProfilePage { props } => format!("{} | rssc-intract", props.user.username),
            },
        ))
        .c(body().c(div().id("content").c(match content {
            Pages::ProfilePage { props } => ProfilePage(cx, props),
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

#[component]
fn ProfilePage(cx: Scope, props: ProfilePageProps) -> View<SsrNode> {
    let heading = format!(
        "{}'s profile",
        match props.user.display_name {
            Some(display_name) => display_name,
            None => props.user.username,
        }
    );

    view! {cx, h1 {(heading)} }
}
