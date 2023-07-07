use clap::Parser;
use dotenv::dotenv;
mod decode;

#[derive(Parser)]
enum FetcherType {
    Auto,
    Range { start: i64, end: i64 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let beacon_url = read_env();
    handle_fetching(FetcherType::parse(), beacon_url).await?;
    Ok(())
}
fn read_env() -> String {
    dotenv().ok();
    std::env::var("BEACON_CHAIN_URL").expect("BEACON_CHAIN_URL must be set in env file")
}

async fn handle_fetching(fethtype: FetcherType, beacon_url: String) -> Result<(), reqwest::Error> {
    match fethtype {
        FetcherType::Auto => auto_fetch(beacon_url).await,
        FetcherType::Range { start, end } => range_fetch(start, end, beacon_url).await,
    }
}

async fn auto_fetch(beacon_url: String) -> Result<(), reqwest::Error> {
    let current_epoch = reqwest::get(format!("{:}/epoch/finalized", beacon_url))
        .await?
        .json::<decode::Epoch>()
        .await?;
    range_fetch(
        current_epoch.data.epoch - 5,
        current_epoch.data.epoch,
        beacon_url,
    )
    .await?;
    println!("{:?}", current_epoch);
    Ok(())
}

async fn range_fetch(start: i64, end: i64, beacon_url: String) -> Result<(), reqwest::Error> {
    Ok(())
}
