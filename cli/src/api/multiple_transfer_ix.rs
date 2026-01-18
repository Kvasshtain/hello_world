use std::fs;
use solana_sdk::signature::{read_keypair_file, Keypair};
use {
    crate::context::Context,
    anyhow::Result,
    hello_world::{Instruction, State},
    solana_sdk::{pubkey::Pubkey, signature::Signer},
};

pub async fn multiple_transfer_ix<'a>(
    context: &Context<'a>,
    amount: u64,
    mint: Pubkey,
    tos_dir_path: String,
) -> Result<solana_sdk::instruction::Instruction> {
    let mut data = vec![Instruction::MultipleTransfer as u8];

    data.extend(amount.to_le_bytes());

    data.extend(mint.to_bytes());


    let dir = fs::read_dir(tos_dir_path)?;


    let mut tos: Vec<Pubkey> = vec![];

    for entry in dir {
        let entry = entry?;
        let path = entry.path();

        let keypair: Keypair = read_keypair_file(path).unwrap();

        tos.push(keypair.pubkey());
    }

    data.extend(tos.len().to_le_bytes());

    let (signer_key, _seed) =
        State::balance_key(&context.program_id, &context.keypair.pubkey(), &mint);

    let mut pubkeys = vec![];

    for to in tos {
        data.extend(to.to_bytes());

        let (to_key, _seed) = State::balance_key(&context.program_id, &to, &mint);

        pubkeys.push(to_key);
    }

    pubkeys.insert(0, signer_key);

    Ok(context.compose_ix(&data.as_slice(), &pubkeys))
}