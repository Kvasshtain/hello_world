use {
    crate::context::Context,
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

pub async fn native_transfer_from<'a>(
    context: &Context<'a>,
    amount: u64,
    seed: String,
    from: Pubkey,
    to: Pubkey,
) -> Result<Vec<Signature>> {
    let mut data = vec![Instruction::TransferFrom as u8];

    data.extend(to.to_bytes());

    data.extend(amount.to_le_bytes());

    data.extend(seed.as_bytes());

    let ix = context.compose_ix(&data.as_slice(), &[&from, &to]);

    let tx = context.compose_tx(&[ix]).await?;

    Ok(vec![context
        .client
        .send_and_confirm_transaction(&tx)
        .await?])
}
