use std::fs;
use solana_sdk::signature::{read_keypair_file, Keypair, Signature};
use solana_sdk::transaction::Transaction;
use {
    crate::context::Context,
    anyhow::Result,
    solana_sdk::{pubkey::Pubkey, signature::Signer},
};
use crate::api::lock_ix::lock_ix;

const MAX_COUNT: usize = 15;

pub async fn batch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    mut pubkeys: Vec<Pubkey>,
    func: fn(&Context,Pubkey, Pubkey) -> solana_sdk::instruction::Instruction,
) -> Result<Vec<Signature>> {
    if pubkeys.is_empty() {
        return Ok(vec![]);
    }

    let ixs = pubkeys
        .into_iter()
        .map(|u| func(&context, mint, u))
        .collect::<Vec<_>>();

    let mut txs = Vec::new();
    let mut futs = vec![];

    for ix in ixs {
        let tx = context.compose_tx(&[ix]).await?;
        txs.push(tx); // сохраняем транзакцию
    }

    for tx in &txs {
        futs.push(context.client.send_and_confirm_transaction(tx));
    }

    // let txs: Vec<Transaction> = futures::future::try_join_all(
    //     ixs.into_iter().map(|ix| context.compose_tx(&[ix]))
    // ).await?;
    //
    // let futs: Vec<_> = txs
    //     .iter()
    //     .map(|tx| context.client.send_and_confirm_transaction(tx))
    //     .collect();

    let results = futures_util::future::join_all(futs)
        .await
        .into_iter()
        //.collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(results)
}

pub async fn multiple_transfer<'a>(
    context: &Context<'a>,
    amount: u64,
    mint: Pubkey,
    tos_dir_path: String,
) -> Result<Vec<Signature>> {
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
        //let entry = entry?;
        let path = entry.path();
        let keypair: Keypair = read_keypair_file(path).unwrap();
        let to = keypair.pubkey();
        tos.push(to);
    }

    let mut sigs = vec![];

    for i in Context::into_chunks(tos, crate::api::distribute::CHUNK_SIZE) {
        sigs.push(batch(context.clone(), mint, i, lock_ix as fn(&Context,Pubkey, Pubkey) -> solana_sdk::instruction::Instruction).await?)
    }


    //^!!!!!!!!!!!!!!!!!!!!!!!!!


    // for i in Context::into_chunks(tos.clone(), crate::api::distribute::CHUNK_SIZE) {
    //     sigs.push(batch(context.clone(), mint, i, unlock_ix as fn(&Context,Pubkey, Pubkey) -> solana_sdk::instruction::Instruction).await?)
    // }

    let sigs = sigs.into_iter().flatten().collect::<Vec<_>>();

    Ok(sigs)
}