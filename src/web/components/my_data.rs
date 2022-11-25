use sycamore::prelude::*;

use crate::{database::user::User, web::language::Translation};

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let username = props.user.username;
    let displayname = props.user.display_name.unwrap_or("".into());

    view! {cx,
        div {
            h1 {(props.translation.my_data)}
            p {(props.translation.my_data_description)}
        }
        table {
            tr {
                th {(props.translation.data)}
                th {(props.translation.value)}
                th {(props.translation.description)}
            }
            // username
            tr {
                td {(props.translation.username)}
                td {(username)}
            }
            tr {
                td {(props.translation.display_name)}
                td {(displayname)}
            }
            tr {
                td {(props.translation.language)}
                td {(props.translation.code)}
                td {(props.translation.language_description)}
            }
            tr {
                td {(props.translation.password)}
                td {(props.translation.redacted_for_your_safety)}
                td {(props.translation.password_description)}
            }
            tr {
                td {(props.translation.session_token)}
                td {(props.translation.redacted_for_your_safety)}
                td {(props.translation.session_token_description)}
            }
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
    pub user: User,
}
