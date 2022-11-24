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
                th {(props.translation.username)}
                th {(username)}
            }
            tr {
                th {(props.translation.display_name)}
                th {(displayname)}
            }
            tr {
                th {(props.translation.language)}
                th {(props.translation.code)}
                th {(props.translation.language_description)}
            }
            tr {
                th {(props.translation.password)}
                th {(props.translation.redacted_for_your_safety)}
                th {(props.translation.password_description)}
            }
            tr {
                th {(props.translation.session_token)}
                th {(props.translation.redacted_for_your_safety)}
                th {(props.translation.session_token_description)}
            }
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
    pub user: User,
}
