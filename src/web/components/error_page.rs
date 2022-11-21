use sycamore::prelude::*;

use crate::web::language::Translation;

pub enum Messages {
    FailedLogout,
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
    pub message: Messages,
}

#[component]
fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    view! {cx,
        p {"what the fuck happened"}
    }
}
