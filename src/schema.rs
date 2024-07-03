// @generated automatically by Diesel CLI.

diesel::table! {
    block_summaries (id) {
        id -> Int4,
        value -> Numeric,
        parent_hash -> Text,
        slot -> Int8,
        builder_pubkey -> Text,
        received_at -> Timestamp,
    }
}
