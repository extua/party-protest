// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
        location -> Nullable<Text>,
        published -> Bool,
    }
}
