use crate::{types::Slots, Pool};
use sqlx::{Postgres, Row};

/// Insert slots to db.
/// - `pool`: Db connection pool to dump data from api.
/// - `slots`: Slots per epoch.
pub async fn insert_slot_to_db(pool: Pool<Postgres>, slots: Slots) -> Result<(), sqlx::Error> {
    let query = "INSERT INTO SLOTS (attestationscount, attesterslashingscount, blockroot, depositscount, epoch, eth1data_blockhash, eth1data_depositcount, eth1data_depositroot, 
        exec_base_fee_per_gas, exec_block_hash, exec_block_number, exec_extra_data, exec_fee_recipient, exec_gas_limit, exec_gas_used, exec_logs_bloom, exec_parent_hash, exec_random, 
        exec_receipts_root, exec_state_root, exec_timestamp, exec_transactions_count, graffiti, graffiti_text, parentroot, proposer, proposerslashingscount, randaoreveal, 
        signature, slot, stateroot, status, syncaggregate_bits, syncaggregate_participation, syncaggregate_signature, voluntaryexitscount, withdrawalcount) VALUES ($1, $2, $3, $4, 
        $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18 , $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36, $37)";

    for slot in slots.data {
        sqlx::query(query)
            .bind(&slot.attestationscount)
            .bind(&slot.attesterslashingscount)
            .bind(&slot.blockroot)
            .bind(&slot.depositscount)
            .bind(&slot.epoch)
            .bind(&slot.eth1data_blockhash)
            .bind(&slot.eth1data_depositcount)
            .bind(&slot.eth1data_depositroot)
            .bind(&slot.exec_base_fee_per_gas)
            .bind(&slot.exec_block_hash)
            .bind(&slot.exec_block_number)
            .bind(&slot.exec_extra_data)
            .bind(&slot.exec_fee_recipient)
            .bind(&slot.exec_gas_limit)
            .bind(&slot.exec_gas_used)
            .bind(&slot.exec_logs_bloom)
            .bind(&slot.exec_parent_hash)
            .bind(&slot.exec_random)
            .bind(&slot.exec_receipts_root)
            .bind(&slot.exec_state_root)
            .bind(&slot.exec_timestamp)
            .bind(&slot.exec_transactions_count)
            .bind(&slot.graffiti)
            .bind(&slot.graffiti_text)
            .bind(&slot.parentroot)
            .bind(&slot.proposer)
            .bind(&slot.proposerslashingscount)
            .bind(&slot.randaoreveal)
            .bind(&slot.signature)
            .bind(&slot.slot)
            .bind(&slot.stateroot)
            .bind(&slot.status)
            .bind(&slot.syncaggregate_bits)
            .bind(&slot.syncaggregate_participation)
            .bind(&slot.syncaggregate_signature)
            .bind(&slot.voluntaryexitscount)
            .bind(&slot.withdrawalcount)
            .execute(&pool)
            .await?;
    }
    Ok(())
}

pub async fn get_slot_participation(pool: Pool<Postgres>) -> Result<Vec<(i64, f64)>, sqlx::Error> {
    let query = "SELECT slot, syncaggregate_participation from slots";

    let result = sqlx::query(query).fetch_all(&pool).await?;
    let slot_participation: Vec<(i64, f64)> =
        result.iter().map(|row| (row.get(0), row.get(1))).collect();
    Ok(slot_participation)
}
