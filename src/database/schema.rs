// @generated automatically by Diesel CLI.

diesel::table! {
    feeds (id) {
        id -> Text,
        url -> Text,
        title -> Nullable<Text>,
        last_updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    follows (username, following) {
        username -> Text,
        following -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Text,
        url -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        feed_id -> Nullable<Text>,
        time -> Timestamp,
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
    shares (post_id, username) {
        post_id -> Text,
        username -> Text,
        user_comment -> Nullable<Text>,
        time -> Timestamp,
    }
}

diesel::table! {
    subscriptions (feed_id, username) {
        feed_id -> Text,
        username -> Text,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        tag -> Text,
        username -> Text,
        post_id -> Text,
    }
}

diesel::table! {
    users (username) {
        username -> Text,
        display_name -> Nullable<Text>,
        hash -> Nullable<Text>,
        salt -> Nullable<Text>,
    }
}

diesel::joinable!(follows -> users (following));

diesel::allow_tables_to_appear_in_same_query!(
    feeds,
    follows,
    posts,
    sessionid,
    shares,
    subscriptions,
    tags,
    users,
);
