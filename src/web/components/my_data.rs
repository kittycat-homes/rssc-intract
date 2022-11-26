use sycamore::prelude::*;

use crate::{database::user::User, web::language::Translation};

#[component]
pub fn Page(cx: Scope, props: Props) -> View<SsrNode> {
    let authorized: View<SsrNode> = match props.user {
        None => view! {cx, },
        Some(user) => {
            let username = user.username;
            let displayname: View<SsrNode> = match user.display_name {
                None => view! {cx, },
                Some(name) => view! {cx,
                    tr {
                        td {(props.translation.display_name)}
                        td {(name)}
                        td {}
                    }
                },
            };
            view! {cx,
                tr {
                    td {(props.translation.username)}
                    td {(username)}
                    td {}
                }
                (displayname)
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
    };

    view! {cx,
        div (class="p-4 lg:p-8") {
            h1 {(props.translation.my_data)}
            p {(props.translation.my_data_description)}
            table (class="table-auto") {
                thead {
                    tr {
                        th {(props.translation.data)}
                        th {(props.translation.value)}
                        th {(props.translation.description)}
                    }
                }
                tbody {
                    tr {
                        td {(props.translation.language)}
                        td {(props.translation.code)}
                        td {(props.translation.language_description)}
                    }
                    (authorized)
                }
            }
        }
    }
}

#[derive(Prop)]
pub struct Props {
    pub translation: Translation,
    pub user: Option<User>,
}
