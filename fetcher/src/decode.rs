use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
    data: SlotData,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlotData {
    attestationscount: i64,
    attesterslashingscount: i64,
    blockroot: String,
    depositscount: i64,
    epoch: i64,
    eth1data_blockhash: String,
    eth1data_depositcount: i64,
    eth1data_depositroot: String,
    exec_base_fee_per_gas: i64,
    exec_block_hash: String,
    exec_block_number: i64,
    exec_extra_data: String,
    exec_fee_recipient: String,
    exec_gas_limit: i64,
    exec_gas_used: i64,
    exec_logs_bloom: String,
    exec_parent_hash: String,
    exec_random: String,
    exec_receipts_root: String,
    exec_state_root: String,
    exec_timestamp: i64,
    exec_transactions_count: i64,
    graffiti: String,
    graffiti_text: String,
    parentroot: String,
    proposer: i64,
    proposerslashingscount: i64,
    randaoreveal: String,
    signature: String,
    slot: i64,
    stateroot: String,
    status: String,
    syncaggregate_bits: String,
    syncaggregate_participation: i64,
    syncaggregate_signature: String,
    voluntaryexitscount: i64,
    withdrawalcount: i64,
}

/*
epoch -



  "data": {
    "attestationscount": i64,
    "attesterslashingscount": i64,
    "averagevalidatorbalance": i64,
    "blockscount": i64,
    "depositscount": i64,
    "eligibleether": i64,
    "epoch": i64,
    "finalized": true,
    "globalparticipationrate": i64,
    "missedblocks": i64,
    "orphanedblocks": i64,
    "proposedblocks": i64,
    "proposerslashingscount": i64,
    "rewards_exported": i64,
    "scheduledblocks": i64,
    "totalvalidatorbalance": i64,
    "ts": i64,
    "validatorscount": i64,
    "voluntaryexitscount": i64,
    "votedether": i64,
    "withdrawalcount": i64
  },
  "status": String




  slot -



  {
  "data": [
    {
      "attestationscount": i64,
      "attesterslashingscount": i64,
      "blockroot": String,
      "depositscount": i64,
      "epoch": i64,
      "eth1data_blockhash": String,
      "eth1data_depositcount": i64,
      "eth1data_depositroot": String,
      "exec_base_fee_per_gas": i64,
      "exec_block_hash": String,
      "exec_block_number": i64,
      "exec_extra_data": String,
      "exec_fee_recipient": String,
      "exec_gas_limit": i64,
      "exec_gas_used": i64,
      "exec_logs_bloom": String,
      "exec_parent_hash": String,
      "exec_random": String,
      "exec_receipts_root": String,
      "exec_state_root": String,
      "exec_timestamp": i64,
      "exec_transactions_count": i64,
      "graffiti": String,
      "graffiti_text": String,
      "parentroot": String,
      "proposer": i64,
      "proposerslashingscount": i64,
      "randaoreveal": String,
      "signature": String,
      "slot": i64,
      "stateroot": String,
      "status": String,
      "syncaggregate_bits": String,
      "syncaggregate_participation": i64,
      "syncaggregate_signature": String,
      "voluntaryexitscount": i64,
      "withdrawalcount": i64
    }
  ],
  "status": String
}



 */
