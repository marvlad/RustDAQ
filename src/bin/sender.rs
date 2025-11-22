use service_discovery_rs::run_multicast_sender;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Rust ServiceDiscovery Sender starting...");
    run_multicast_sender().await?;
    Ok(())
}

