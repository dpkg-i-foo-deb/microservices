// @generated automatically by Diesel CLI.

diesel::table! {
    sessions (id) {
        #[max_length = 255]
        id -> Varchar,
        start_date -> Timestamp,
        end_date -> Nullable<Timestamp>,
        #[max_length = 255]
        user_id -> Varchar,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        username -> Varchar,
    }
}

diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
