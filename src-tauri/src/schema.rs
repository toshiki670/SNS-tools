// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Text,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
