mod config;
mod event_detector;
mod executor;
mod logger;
mod rpc;
mod transaction_builder;
mod wallet;

use anyhow::Result;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    logger::init();

    let app_config = config::AppConfig::load()?;
    let wallet = wallet::Wallet::load(&app_config.keypair_path)?;
    let rpc_client = rpc::RpcService::connect(&app_config.rpc_url)?;
    let detector = event_detector::EventDetector::new(app_config.strategy.clone());
    let executor = executor::Executor::new(rpc_client, wallet, app_config.strategy.clone());

    loop {
        if detector.event_detected().await? {
            let transaction = transaction_builder::build_transaction(&executor.wallet(), &app_config.strategy)?;
            executor.execute(transaction).await?;
        }

        tokio::time::sleep(Duration::from_secs(app_config.strategy.check_interval_secs)).await;
    }
}
