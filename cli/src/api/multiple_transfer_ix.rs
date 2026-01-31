use std::fs;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{read_keypair_file, Keypair, Signer};
use hello_world::{Instruction, State};
use crate::context::Context;

pub const MAX_COUNT: usize = 15;

pub async fn multiple_transfer_ix<'a>(
    context: &Context<'a>,
    amount: u64,
    mint: Pubkey,
    tos: Vec<Pubkey>,
) -> anyhow::Result<solana_sdk::instruction::Instruction> {
    let mut data = vec![Instruction::MultipleTransfer as u8];

    data.extend(amount.to_le_bytes());

    data.extend(mint.to_bytes());


    //let dir = fs::read_dir(tos_dir_path)?;


    //let mut tos: Vec<Pubkey> = vec![];

    // for entry in dir {
    //     let entry = entry?;
    //     let path = entry.path();
    //
    //     let keypair: Keypair = read_keypair_file(path).unwrap();
    //
    //     tos.push(keypair.pubkey());
    // }

    if MAX_COUNT < tos.len() {
        return Err(anyhow::Error::msg("Too many accounts to transfer"));
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