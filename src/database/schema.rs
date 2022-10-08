// @generated automatically by Diesel CLI.

diesel::table! {
    feeds (url) {
        url -> Text,
        title -> Nullable<Text>,
    }
}

diesel::table! {
    follows (username, following) {
        username -> Text,
        following -> Text,
    }
}

diesel::table! {
    posts (url) {
        url -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        feed_url -> Nullable<Text>,
    }
}

diesel::table! {
    sessionid (id) {
        id -> Text,
        username -> Text,
        last_active -> Nullable<Timestamp>,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    shares (post_url, username) {
        post_url -> Text,
        username -> Text,
    }
}

diesel::table! {
    subscriptions (feed_url, username) {
        feed_url -> Text,
        username -> Text,
    }
}

diesel::table! {
    users (username) {
        username -> Text,
        display_name -> Nullable<Text>,
        pfp -> Nullable<Text>,
        hash -> Nullable<Text>,
        salt -> Nullable<Text>,
    }
}

diesel::joinable!(follows -> users (following));
diesel::joinable!(shares -> posts (post_url));

diesel::allow_tables_to_appear_in_same_query!(
    feeds,
    follows,
    posts,
    sessionid,
    shares,
    subscriptions,
    users,
);
