// @generated automatically by Diesel CLI.

diesel::table! {
    peers (id) {
        id -> Nullable<Integer>,
        client_id -> Text,
        ip -> Text,
        device_type -> Text,
        device -> Text,
        device_token -> Nullable<Text>,
    }
}
