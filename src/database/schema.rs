table! {
    captions (id) {
        id -> Int4,
        content -> Text,
        start_time -> Nullable<Interval>,
        end_time -> Nullable<Interval>,
        media -> Int4,
    }
}

table! {
    categories (id) {
        id -> Int4,
        term -> Text,
        label -> Nullable<Text>,
        feed -> Nullable<Text>,
        post -> Nullable<Text>,
    }
}

table! {
    feeds (id) {
        id -> Text,
        url -> Text,
        title -> Nullable<Text>,
        last_updated -> Timestamp,
        description -> Nullable<Text>,
        language -> Nullable<Text>,
    }
}

table! {
    images (id) {
        id -> Int4,
        uri -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        media -> Int4,
        feed -> Nullable<Text>,
    }
}

table! {
    links (id) {
        id -> Int4,
        href -> Nullable<Text>,
        title -> Nullable<Text>,
        feed -> Nullable<Text>,
        post -> Nullable<Text>,
    }
}

table! {
    medias (id) {
        id -> Int4,
        uri -> Nullable<Text>,
        title -> Nullable<Text>,
        mime -> Nullable<Text>,
        description -> Nullable<Text>,
        post -> Nullable<Text>,
    }
}

table! {
    posts (id) {
        id -> Text,
        url -> Text,
        title -> Nullable<Text>,
        description -> Nullable<Text>,
        feed_id -> Nullable<Text>,
        time -> Timestamp,
        content -> Nullable<Text>,
        summary -> Nullable<Text>,
    }
}

table! {
    sessionid (id) {
        id -> Text,
        username -> Text,
        last_active -> Timestamp,
        name -> Nullable<Text>,
    }
}

table! {
    subscriptions (feed_id, username) {
        feed_id -> Text,
        username -> Text,
    }
}

table! {
    users (username) {
        username -> Text,
        display_name -> Nullable<Text>,
        hash -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    captions,
    categories,
    feeds,
    images,
    links,
    medias,
    posts,
    sessionid,
    subscriptions,
    users,
);
