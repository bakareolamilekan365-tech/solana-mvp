use anyhow::{Context, Result};
use solana_sdk::signature::{read_keypair_file, Keypair};

pub struct Wallet {
    keypair: Keypair,
}

impl Wallet {
    pub fn load(path: &str) -> Result<Self> {
        let keypair = read_keypair_file(path).with_context(|| format!("failed to load keypair from {}", path))?;
        Ok(Self { keypair })
    }

    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }
}
