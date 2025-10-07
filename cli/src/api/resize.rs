use {
    crate::context::Context,
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

pub async fn resize<'a>(context: Context<'a>, pda: Pubkey, size: u64) -> Result<Signature> {
    let mut data = vec![Instruction::Resize as u8];

    data.extend(size.to_le_bytes());

    let ix = context.compose_ix(&data.as_slice(), &[&pda]);

    let tx = context.compose_tx(&[ix]).await?;

    Ok(context.client.send_and_confirm_transaction(&tx).await?)
}
