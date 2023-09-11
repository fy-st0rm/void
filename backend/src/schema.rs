// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Text,
        owner -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    users,
);
