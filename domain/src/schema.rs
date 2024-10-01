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
