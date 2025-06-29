use ream_bls::PublicKey;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct ProposerDuty {
    #[serde(rename = "pubkey")]
    pub public_key: PublicKey,
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_index: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct AttesterDuty {
    #[serde(rename = "pubkey")]
    pub public_key: PublicKey,
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_index: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub committee_index: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub committees_at_slot: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_committee_index: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    pub slot: u64,
}

#[derive(Debug, Deserialize, Serialize, Encode, Decode)]
pub struct SyncCommitteeDuty {
    #[serde(rename = "pubkey")]
    pub public_key: PublicKey,
    #[serde(with = "serde_utils::quoted_u64")]
    pub validator_index: u64,
    pub validator_sync_committee_indices: Vec<u64>,
}
