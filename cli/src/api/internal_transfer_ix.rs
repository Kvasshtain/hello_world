use {
    crate::context::Context,
    anyhow::Result,
    hello_world::{Instruction, State},
    solana_sdk::{pubkey::Pubkey, signature::Signer},
};

pub async fn internal_transfer_ix<'a>(
    context: &Context<'a>,
    amount: u64,
    mint: Pubkey,
    to: Pubkey,
) -> Result<solana_sdk::instruction::Instruction> {
    let mut data = vec![Instruction::InternalTransfer as u8];

    data.extend(amount.to_le_bytes());

    data.extend(mint.to_bytes());

    data.extend(to.to_bytes());

    let (signer_key, _seed) =
        State::balance_key(&context.program_id, &context.keypair.pubkey(), &mint);
    let (to_key, _seed) = State::balance_key(&context.program_id, &to, &mint);

    Ok(context.compose_ix(&data.as_slice(), &[&signer_key, &to_key]))
}
