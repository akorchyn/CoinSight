// @generated automatically by Diesel CLI.

diesel::table! {
    notification_preferences (id) {
        id -> Int4,
        notification_id -> Int4,
        notification_method -> Varchar,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    notifications (id) {
        id -> Int4,
        user_id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        source -> Varchar,
        value_change -> Nullable<Numeric>,
        percent_change -> Nullable<Numeric>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        is_active -> Nullable<Bool>,
        cryptocurrency -> Varchar,
        current_price -> Numeric,
        name -> Varchar,
    }
}

diesel::table! {
    telegram_auth (id) {
        id -> Int4,
        user_id -> Int4,
        telegram_id -> Nullable<Int8>,
        auth_code -> Varchar,
    }
}

diesel::table! {
    tokens (id) {
        id -> Int4,
        user_id -> Int4,
        token -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        login -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        default_notification_method -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(notification_preferences -> notifications (notification_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(telegram_auth -> users (user_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    notification_preferences,
    notifications,
    telegram_auth,
    tokens,
    users,
);
