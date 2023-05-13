// @generated automatically by Diesel CLI.

diesel::table! {
    aggregated_prices (id) {
        id -> Int4,
        crypto_id -> Int4,
        currency_id -> Int4,
        median_price -> Numeric,
        first_quartile_price -> Numeric,
        third_quartile_price -> Numeric,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    cryptocurrencies (id) {
        id -> Int4,
        name -> Varchar,
        symbol -> Varchar,
        description -> Text,
    }
}

diesel::table! {
    currencies (id) {
        id -> Int4,
        name -> Varchar,
        symbol -> Varchar,
    }
}

diesel::table! {
    prices (id) {
        id -> Int4,
        crypto_id -> Int4,
        source_id -> Int4,
        currency_id -> Int4,
        price -> Numeric,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    source_crypto_mappings (id) {
        id -> Int4,
        crypto_id -> Int4,
        source_id -> Int4,
        source_key -> Varchar,
    }
}

diesel::table! {
    sources (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(aggregated_prices -> cryptocurrencies (crypto_id));
diesel::joinable!(aggregated_prices -> currencies (currency_id));
diesel::joinable!(prices -> cryptocurrencies (crypto_id));
diesel::joinable!(prices -> currencies (currency_id));
diesel::joinable!(prices -> sources (source_id));
diesel::joinable!(source_crypto_mappings -> cryptocurrencies (crypto_id));
diesel::joinable!(source_crypto_mappings -> sources (source_id));

diesel::allow_tables_to_appear_in_same_query!(
    aggregated_prices,
    cryptocurrencies,
    currencies,
    prices,
    source_crypto_mappings,
    sources,
);
