use {
    crate::context::Context,
    anyhow::Result,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
};

pub async fn assign<'a>(context: Context<'a>, seed: String, owner: Pubkey) -> Result<Signature> {
    let mut data = vec![Instruction::Assign as u8];

    data.extend(owner.to_bytes());

    data.extend(seed.as_bytes());

    let ix = context.compose_ix(&data.as_slice(), &[&owner]);

    let tx = context.compose_tx(&[ix]).await?;

    Ok(context.client.send_and_confirm_transaction(&tx).await?)
}
