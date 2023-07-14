use dotenv::dotenv;
use indexer::{ConnectionPool, FetchType, Parser};
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

/// Read env from dot env file.
/// Parameters:
/// - `env_var`: Name of env variable from dot env file.
fn read_env(env_var: &str) -> String {
    std::env::var(env_var).expect("ENV VARIABLE must be set.")
}
