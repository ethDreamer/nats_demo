use crate::types::BlockSummary;
use super::schema::block_summaries;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = block_summaries)]
pub struct BlockSummaryEntry {
    #[diesel(sql_type = diesel::sql_types::Numeric)]
    value: BigDecimal,
    parent_hash: String,
    slot: i64,
    builder_pubkey: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    received_at: NaiveDateTime,
}

impl BlockSummaryEntry {
    pub fn from_components(block_summary: BlockSummary, received_at: NaiveDateTime) -> Self {
        Self {
            value: block_summary.value_as_bigdecimal(),
            parent_hash: block_summary.parent_hash.to_string(),
            slot: block_summary.slot as i64,
            builder_pubkey: block_summary.builder_pubkey.to_string(),
            received_at,
        }
    }
}
