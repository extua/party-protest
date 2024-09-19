// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        start_time -> Integer,
        end_time -> Nullable<Integer>,
        location -> Nullable<Text>,
    }
}

diesel::table! {
    users (username) {
        username -> Nullable<Text>,
        password -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,
    users,
);
