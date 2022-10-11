// @generated automatically by Diesel CLI.

diesel::table! {
    feeds (id) {
        id -> Text,
        url -> Text,
        title -> Nullable<Text>,
        last_updated -> Timestamp,
    }
}

diesel::table! {
    follows (follower, followed) {
        follower -> Text,
        followed -> Text,
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
        last_active -> Timestamp,
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
        value -> Text,
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
