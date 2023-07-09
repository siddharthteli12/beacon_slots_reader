use sqlx::Postgres;

use crate::{types::Slots, Pool};

/// Insert slots to db.
/// - `pool`: Db connection pool to dump data from api.
/// - `slots`: Slots per epoch.
pub async fn insert_slot_to_db(pool: Pool<Postgres>, slots: Slots) -> Result<(), sqlx::Error> {
    Ok(())
}
