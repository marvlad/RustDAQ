use service_discovery_rs::run_multicast_receiver;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rust multicast receiver started");
    run_multicast_receiver().await
}

