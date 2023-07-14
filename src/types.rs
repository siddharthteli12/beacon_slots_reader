use serde::{Deserialize, Serialize};

// Epoch response struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Epoch {
    pub status: String,
    pub data: EpochData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpochData {
    pub attestationscount: i64,
    pub attesterslashingscount: i64,
    pub averagevalidatorbalance: i64,
    pub blockscount: i64,
    pub depositscount: i64,
    pub eligibleether: i64,
    pub epoch: i64,
    pub finalized: bool,
    pub globalparticipationrate: f64,
    pub missedblocks: i64,
    pub orphanedblocks: i64,
    pub proposedblocks: i64,
    pub proposerslashingscount: i64,
    pub rewards_exported: bool,
    pub scheduledblocks: i64,
    pub totalvalidatorbalance: i64,
    pub ts: String,
    pub validatorscount: i64,
    pub voluntaryexitscount: i64,
    pub votedether: i64,
    pub withdrawalcount: i64,
}

// Slots response struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Slots {
    pub status: String,
    pub data: Vec<SlotData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlotData {
    pub attestationscount: i64,
    pub attesterslashingscount: i64,
    pub blockroot: String,
    pub depositscount: i64,
    pub epoch: i64,
    pub eth1data_blockhash: Option<String>,
    pub eth1data_depositcount: i64,
    pub eth1data_depositroot: Option<String>,
    pub exec_base_fee_per_gas: i64,
    pub exec_block_hash: Option<String>,
    pub exec_block_number: i64,
    pub exec_extra_data: Option<String>,
    pub exec_fee_recipient: Option<String>,
    pub exec_gas_limit: i64,
    pub exec_gas_used: i64,
    pub exec_logs_bloom: Option<String>,
    pub exec_parent_hash: Option<String>,
    pub exec_random: Option<String>,
    pub exec_receipts_root: Option<String>,
    pub exec_state_root: Option<String>,
    pub exec_timestamp: i64,
    pub exec_transactions_count: i64,
    pub graffiti: Option<String>,
    pub graffiti_text: Option<String>,
    pub parentroot: Option<String>,
    pub proposer: i64,
    pub proposerslashingscount: i64,
    pub randaoreveal: Option<String>,
    pub signature: Option<String>,
    pub slot: i64,
    pub stateroot: Option<String>,
    pub status: Option<String>,
    pub syncaggregate_bits: Option<String>,
    pub syncaggregate_participation: f64,
    pub syncaggregate_signature: Option<String>,
    pub voluntaryexitscount: i64,
    pub withdrawalcount: i64,
}
