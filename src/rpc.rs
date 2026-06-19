use anyhow::{Context, Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{hash::Hash, transaction::Transaction};

#[derive(Clone)]
pub struct RpcService {
    client: RpcClient,
}

impl RpcService {
    pub fn connect(rpc_url: &str) -> Result<Self> {
        let client = RpcClient::new(rpc_url.to_string());
        Ok(Self { client })
    }

    pub fn latest_blockhash(&self) -> Result<Hash> {
        self.client.get_latest_blockhash().context("failed to fetch latest blockhash")
    }

    pub fn send_and_confirm(&self, transaction: &Transaction) -> Result<String> {
        let signature = self
            .client
            .send_and_confirm_transaction(transaction)
            .context("failed to send and confirm transaction")?;
        Ok(signature.to_string())
    }
}
