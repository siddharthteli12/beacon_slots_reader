use dotenv::dotenv;
use indexer::{read_env, ConnectionPool, FetchType, Parser};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();
    let fetch_type = FetchType::parse();
    let fetch = ConnectionPool::new(
        read_env("POSTGRES_URL"),
        read_env("BEACON_CHAIN_URL"),
        fetch_type,
    )
    .await;

    fetch.handle_fetching().await?;

    Ok(())
}
