use {
    crate::{context::Context,
            api::deposit_withdraw::deposit_withdraw,
    },
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

pub async fn deposit<'a>(context: Context<'a>, amount: u64, mint: Pubkey) -> Result<Signature> {
    deposit_withdraw(context, amount, mint, Instruction::Deposit).await
}
