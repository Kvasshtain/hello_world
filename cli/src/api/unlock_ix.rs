use solana_sdk::signature::Signer;
use {
    crate::context::Context, anyhow::Result, hello_world::Instruction, solana_sdk::pubkey::Pubkey,
};
use hello_world::State;

pub async fn unlock_ix<'a>(
    context: &Context<'a>,
    mint: Pubkey,
    pubkey: Pubkey,
) -> Result<solana_sdk::instruction::Instruction> {
    let mut data = vec![Instruction::Unlock as u8];

    data.extend(mint.to_bytes());

    data.extend(pubkey.to_bytes());

    let (signer_key, _seed) =
        State::balance_key(&context.program_id, &context.keypair.pubkey(), &mint);
    let (balance_key, _seed) = State::balance_key(&context.program_id, &pubkey, &mint);

    Ok(context.compose_ix(&data.as_slice(), &[signer_key, balance_key]))
}