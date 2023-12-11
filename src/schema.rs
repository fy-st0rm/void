// @generated automatically by Diesel CLI.

diesel::table! {
    files (id) {
        id -> Varchar,
        owner -> Varchar,
        name -> Varchar,
        url -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
        hash -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    files,
    users,
);
