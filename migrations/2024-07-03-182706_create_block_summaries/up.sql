-- Your SQL goes here
CREATE TABLE block_summaries (
    id SERIAL PRIMARY KEY,
    value NUMERIC NOT NULL,
    parent_hash TEXT NOT NULL,
    slot BIGINT NOT NULL,
    builder_pubkey TEXT NOT NULL,
    received_at TIMESTAMP NOT NULL
);
