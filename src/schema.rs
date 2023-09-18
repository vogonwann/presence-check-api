// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        last_name -> Text,
        created_at -> Timestamp,
    }
}
