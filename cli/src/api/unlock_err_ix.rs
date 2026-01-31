use hello_world::State;
use solana_sdk::signature::Signer;
use {
    crate::context::Context, hello_world::Instruction, solana_sdk::pubkey::Pubkey,
};

pub fn unlock_err_ix(
    context: &Context,
    mint: Pubkey,
    pubkey: Pubkey,
) -> solana_sdk::instruction::Instruction {
    let mut data = vec![Instruction::UnlockErr as u8];

    data.extend(mint.to_bytes());

    data.extend(pubkey.to_bytes());

    let (signer_key, _seed) =
        State::balance_key(&context.program_id, &context.keypair.pubkey(), &mint);
    let (balance_key, _seed) = State::balance_key(&context.program_id, &pubkey, &mint);

    context.compose_ix(&data.as_slice(), &[signer_key, balance_key])
}