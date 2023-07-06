use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let beacon_chain_url =
        std::env::var("BEACON_CHAIN_URL").expect("BEACON_CHAIN_URL must be set in env file");
}
