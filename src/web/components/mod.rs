use rocket::response::content::RawHtml;
use sycamore::reactive::Scope;
use sycamore::view::View;
use sycamore::web::{Html, SsrNode};
use sycamore::{component, render_to_string, view};

pub fn wrap_and_render(title: &str, content: View<SsrNode>) -> RawHtml<String> {
    let title = format!("{} | rssc-intract", title);
    RawHtml(render_to_string(|cx| {
        view! { cx,
            r"<!DOCTYPE html>>"
            Head(title)
            div (class="content") {(content)}
        }
    }))
}

#[component]
fn Head(cx: Scope, title: String) -> View<SsrNode> {
    view! {cx, head {
        meta (name="viewport")
        title {(title)}
    } }
}
