use {
    crate::context::Context,
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

pub async fn native_transfer<'a>(
    context: &Context<'a>,
    amount: u64,
    to: Pubkey,
) -> Result<Signature> {
    let mut data = vec![Instruction::Transfer as u8];

    data.extend(to.to_bytes());

    data.extend(amount.to_le_bytes());

    let ix = context.compose_ix(&data.as_slice(), &[&to]);

    let tx = context.compose_tx(&[ix]).await?;

    Ok(context.client.send_and_confirm_transaction(&tx).await?)
}
