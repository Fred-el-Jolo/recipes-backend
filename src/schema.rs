// @generated automatically by Diesel CLI.

diesel::table! {
    likes (id) {
        id -> Integer,
        created_at -> Timestamp,
        tweet_id -> Nullable<Integer>,
    }
}

diesel::table! {
    tweets (id) {
        id -> Integer,
        created_at -> Timestamp,
        message -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    likes,
    tweets,
);