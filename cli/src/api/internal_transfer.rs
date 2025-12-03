use {
    crate::context::Context,
    anyhow::Result,
    hello_world::{Instruction, State},
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Signature, Signer},
    },
};

pub async fn internal_transfer<'a>(
    context: Context<'a>,
    amount: u64,
    mint: Pubkey,
    to: Pubkey,
) -> Result<Signature> {
    let mut data = vec![Instruction::InternalTransfer as u8];

    data.extend(amount.to_le_bytes());

    data.extend(mint.to_bytes());

    data.extend(to.to_bytes());

    let (signer_key, _bump, _seed) =
        State::balance_pubkey_bump(&context.program_id, &context.keypair.pubkey(), &mint);
    let (to_key, _bump, _seed) = State::balance_pubkey_bump(&context.program_id, &to, &mint);

    let ix = context.compose_ix(&data.as_slice(), &[&signer_key, &to_key]);

    let tx = context.compose_tx(&[ix]).await?;

    Ok(context.client.send_and_confirm_transaction(&tx).await?)
}
