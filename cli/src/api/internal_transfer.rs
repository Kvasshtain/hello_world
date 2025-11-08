use {
    crate::context::Context,
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature, signature::Signer},
};

pub async fn internal_transfer<'a>(
    context: Context<'a>,
    amount: u64,
    mint: Pubkey,
    to: Pubkey,
) -> Result<Vec<Signature>> {
    let mut data = vec![Instruction::InternalTransfer as u8];

    data.extend(amount.to_le_bytes());

    data.extend(mint.to_bytes());

    data.extend(to.to_bytes());

    let signer_key = context.balance_info(&context.keypair.pubkey(), &mint);

    let to_key = context.balance_info(&to, &mint);

    let ix = context.compose_ix(&data.as_slice(), &[&signer_key, &to_key]);

    let tx = context.compose_tx(&[ix]).await?;

    Ok(vec![context
        .client
        .send_and_confirm_transaction(&tx)
        .await?])
}
