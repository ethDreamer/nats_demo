use bigdecimal::BigDecimal;
use consensus_types::{Hash256, test_utils::TestRandom, Uint256, PublicKeyBytes};
use ssz_derive::{Encode, Decode};
use std::str::FromStr;
use test_random_derive::TestRandom;

#[derive(Debug, Clone, Encode, Decode, TestRandom)]
pub struct BlockSummary {
    pub value: Uint256,
    pub parent_hash: Hash256,
    pub slot: u64,
    pub builder_pubkey: PublicKeyBytes,
}

impl BlockSummary {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::random_for_test(&mut rng)
    }

    pub fn value_as_bigdecimal(&self) -> BigDecimal {
        BigDecimal::from_str(self.value.to_string().as_str())
            .expect("sould work")
    }
}