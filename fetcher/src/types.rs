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
    status: String,
    data: Vec<SlotData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlotData {
    attestationscount: i64,
    attesterslashingscount: i64,
    blockroot: String,
    depositscount: i64,
    epoch: i64,
    eth1data_blockhash: Option<String>,
    eth1data_depositcount: i64,
    eth1data_depositroot: Option<String>,
    exec_base_fee_per_gas: i64,
    exec_block_hash: Option<String>,
    exec_block_number: i64,
    exec_extra_data: Option<String>,
    exec_fee_recipient: Option<String>,
    exec_gas_limit: i64,
    exec_gas_used: i64,
    exec_logs_bloom: Option<String>,
    exec_parent_hash: Option<String>,
    exec_random: Option<String>,
    exec_receipts_root: Option<String>,
    exec_state_root: Option<String>,
    exec_timestamp: i64,
    exec_transactions_count: i64,
    graffiti: Option<String>,
    graffiti_text: Option<String>,
    parentroot: Option<String>,
    proposer: i64,
    proposerslashingscount: i64,
    randaoreveal: Option<String>,
    signature: Option<String>,
    slot: i64,
    stateroot: Option<String>,
    status: Option<String>,
    syncaggregate_bits: Option<String>,
    syncaggregate_participation: f64,
    syncaggregate_signature: Option<String>,
    voluntaryexitscount: i64,
    withdrawalcount: i64,
}
