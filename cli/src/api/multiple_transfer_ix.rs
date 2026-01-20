use std::fs;
use solana_sdk::signature::{read_keypair_file, Keypair, Signature};
use {
    crate::context::Context,
    anyhow::Result,
    hello_world::{Instruction, State},
    solana_sdk::{pubkey::Pubkey, signature::Signer},
};

const MAX_COUNT: usize = 15;

pub async fn multiple_transfer_ix<'a>(
    context: &Context<'a>,
    amount: u64,
    mint: Pubkey,
    tos_dir_path: String,
) -> Result<Vec<Signature>> {
    let mut data = vec![Instruction::MultipleTransfer as u8];

    data.extend(amount.to_le_bytes());




    let dir = fs::read_dir(tos_dir_path)?;


    let mut tos: Vec<Pubkey> = vec![];

    let mut tos_len: usize = 0;

    

    for entry in dir {

        let entry = entry?;

        let file_type = entry.file_type()?;

        if file_type.is_dir(){
            continue;
        }

        tos_len = tos_len + 1;

        let entry = entry?;
        let path = entry.path();

        let keypair: Keypair = read_keypair_file(path).unwrap();

        let to = keypair.pubkey();

        tos.push(to);

        data.extend(to.to_bytes());
    }

    data.extend(tos_len.to_le_bytes());

    data.extend(mint.to_bytes());
    

    let batch_count = tos_len / MAX_COUNT;


    let mut pubkeys = vec![];


    for batch_index in 0..batch_count {
        data.extend(tos_len.to_le_bytes());

        let (signer_key, _seed) =
            State::balance_key(&context.program_id, &context.keypair.pubkey(), &mint);

        let batch = tos[batch_index * MAX_COUNT..(batch_index + 1) * MAX_COUNT].to_vec();

        for to in batch {
            data.extend(to.to_bytes());

            let (to_key, _seed) = State::balance_key(&context.program_id, &to, &mint);

            pubkeys.push(to_key);
        }

        pubkeys.insert(0, signer_key);

        let ix = context.compose_ix(&data.as_slice(), &pubkeys);

        let tx = context.compose_tx(&[ix]).await?;

        context.client.send_and_confirm_transaction(&tx).await?;
    };

    Ok(sigs)

    //Ok(context.compose_ix(&data.as_slice(), &pubkeys))
}