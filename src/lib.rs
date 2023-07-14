pub mod types;
pub use clap::Parser;
pub mod db;
use log::info;
pub use sqlx::{Pool, Postgres};
// Db pool & other urls.
pub struct ConnectionPool {
    db_pool: Pool<Postgres>,
    beacon_url: String,
    fetch_type: FetchType,
}

// Fetch block type auto or range.
#[derive(Parser)]
pub enum FetchType {
    Auto,
    Range { start: i64, end: i64 },
}

impl ConnectionPool {
    /// Create new connection pool with beacon url & fetch type.
    /// Parameters:
    /// - `db_url`: Db url set in dot env file.
    /// - `beacon_url`: Becaon url to fetch slots from, set in dot env file.
    /// - `fetch_type`: Auto or range fetch.
    pub async fn new(db_url: String, beacon_url: String, fetch_type: FetchType) -> ConnectionPool {
        let db_pool = sqlx::postgres::PgPool::connect(&db_url)
            .await
            .expect("Issue connecting with db");

        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Issue running migration");

        ConnectionPool {
            db_pool,
            beacon_url: beacon_url.to_string(),
            fetch_type,
        }
    }

    // Handle fetching for fetching type.
    pub async fn handle_fetching(self) -> Result<(), reqwest::Error> {
        match self.fetch_type {
            FetchType::Auto => {
                Self::auto_fetch(self).await?;
            }
            FetchType::Range { start, end } => {
                Self::range_fetch(self, start, end).await?;
            }
        }
        Ok(())
    }

    // Fetch slots from 5 blocks from finalised then fetch at fixed interval.
    pub async fn auto_fetch(self) -> Result<(), reqwest::Error> {
        let current_epoch = reqwest::get(format!("{:}/epoch/finalized", self.beacon_url))
            .await?
            .json::<types::Epoch>()
            .await?;
        Self::range_fetch(self, current_epoch.data.epoch - 5, current_epoch.data.epoch).await?;
        info!(
            "Fetching slots, from {:} to {:}",
            current_epoch.data.epoch - 5,
            current_epoch.data.epoch
        );
        Ok(())
    }

    // Fetch from given range.
    /// Parameters:
    /// - `start`: Init epoch no. to start fetching from.
    /// - `end`: Last epoch no. to end fetching at.
    pub async fn range_fetch(self, start: i64, end: i64) -> Result<(), reqwest::Error> {
        for epoch in start..end {
            let url = format!("{:}/epoch/{:}/slots", self.beacon_url, epoch);
            let pool = self.db_pool.clone();
            let slots = get_slots(&url).await?;
            db::insert_slot_to_db(pool, slots)
                .await
                .expect("Error inserting ot db");
        }
        Ok(())
    }
}

// Utility function to get slots from api.
/// Parameters:
/// - `url`: Beacon api url to get slots.
pub async fn get_slots(url: &str) -> Result<types::Slots, reqwest::Error> {
    let slots = reqwest::get(url).await?.json::<types::Slots>().await?;
    Ok(slots)
}

/// Read env from dot env file.
/// Parameters:
/// - `env_var`: Name of env variable from dot env file.
pub fn read_env(env_var: &str) -> String {
    std::env::var(env_var).expect("ENV VARIABLE must be set.")
}
