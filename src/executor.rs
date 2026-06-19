use crate::{config::StrategyConfig, rpc::RpcService, transaction_builder::build_transaction, wallet::Wallet};
use anyhow::Result;
use solana_sdk::transaction::Transaction;

pub struct Executor {
    rpc: RpcService,
    wallet: Wallet,
    strategy: StrategyConfig,
}

impl Executor {
    pub fn new(rpc: RpcService, wallet: Wallet, strategy: StrategyConfig) -> Self {
        Self { rpc, wallet, strategy }
    }

    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    pub async fn execute(&self, transaction: Transaction) -> Result<()> {
        let mut attempts = 0u8;
        loop {
            attempts += 1;
            match self.rpc.send_and_confirm(&transaction) {
                Ok(signature) => {
                    tracing::info!(signature = %signature, attempts, "transaction confirmed");
                    return Ok(());
                }
                Err(error) if attempts <= self.strategy.max_retries => {
                    tracing::warn!(attempts, error = %error, "transaction failed, retrying once");
                }
                Err(error) => return Err(error),
            }
        }
    }
}
