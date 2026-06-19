use crate::{config::StrategyConfig, wallet::Wallet};
use anyhow::Result;
use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
    transaction::Transaction,
    signature::Signer,
};

pub fn build_transaction(wallet: &Wallet, strategy: &StrategyConfig) -> Result<Transaction> {
    let recipient = Pubkey::default();
    let instruction = system_instruction::transfer(
        &wallet.keypair().pubkey(),
        &recipient,
        strategy.transfer_lamports,
    );

    let transaction = Transaction::new_with_payer(&[instruction], Some(&wallet.keypair().pubkey()));
    Ok(transaction)
}
