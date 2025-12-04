use {
    crate::{context::Context,
            api::deposit_withdraw::deposit_withdraw,
    },
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

pub async fn withdraw<'a>(context: Context<'a>, amount: u64, mint: Pubkey) -> Result<Signature> {
    let balance = Context::get_balance(context.clone(), mint).await?;

    if balance < amount {
        return Err(anyhow::Error::msg("Insufficient balance"));
    }

    deposit_withdraw(context, amount, mint, Instruction::Withdraw).await
}
