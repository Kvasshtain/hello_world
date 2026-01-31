use solana_sdk::transaction::{Transaction, TransactionError};
use solana_transaction_status_client_types::UiTransactionEncoding;
use crate::api::unlock_ix::{unlock_ix};
use crate::api::unlock_err_ix::{unlock_err_ix};
use {
    crate::{api::lock_ix::lock_ix, context::Context},
    anyhow::Result,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signature, Signer},
    },
    std::fs,
};
use hello_world::{Instruction, State};
use crate::api::multiple_transfer_ix::multiple_transfer_ix;

async fn process_chunks<'a>(
    context: &Context<'a>,
    mint: Pubkey,
    tos: &[Pubkey],
    func: fn(&Context, Pubkey, Pubkey) -> solana_sdk::instruction::Instruction,
) -> Result<Vec<Vec<Signature>>> {
    let mut sigs = vec![];
    for chunk in Context::into_chunks(tos.to_vec(), crate::api::distribute::CHUNK_SIZE) {
        sigs.push(batch(context.clone(), mint, chunk, func).await?);
    }
    Ok(sigs)
}



pub async fn batch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    pubkeys: Vec<Pubkey>,
    func: fn(&Context, Pubkey, Pubkey) -> solana_sdk::instruction::Instruction,
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

    let results = futures_util::future::join_all(futs)
        .await
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
    let mut accaunts: Vec<Pubkey> = vec![];
    let mut tos: Vec<Pubkey> = vec![];
    let mut tos_len: usize = 0;

    accaunts.push(context.keypair.pubkey());

    for entry in dir {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            continue;
        }

        tos_len = tos_len + 1;
        //let entry = entry?;
        let path = entry.path();
        let keypair: Keypair = read_keypair_file(path).unwrap();
        let to = keypair.pubkey();
        accaunts.push(to);
        tos.push(to);
    }

    let mut sigs = vec![];

    sigs.extend(process_chunks(context, mint, &accaunts, lock_ix).await?);

    let chunks = Context::into_chunks(tos.to_vec(), crate::api::multiple_transfer_ix::MAX_COUNT);

    let mut error: Option<TransactionError> = None;

    for chunk in chunks {
        let ix = multiple_transfer_ix(context, amount, mint, chunk).await?;

        let tx = context.compose_tx(&[ix]).await?;

        let sig: Signature = context.client.send_and_confirm_transaction(&tx).await?;

        let result = context.client.get_transaction(
            &sig,
            UiTransactionEncoding::Json,
        ).await?;

        if let Some(err) = result.transaction.meta.and_then(|m| m.err) {
            error = Option::from(err);
            break;
        }

        sigs.push(vec![sig]);
    }

    if let Some(err) = error {
        sigs.extend(process_chunks(context, mint, &accaunts, unlock_err_ix).await?);
    } else {
        sigs.extend(process_chunks(context, mint, &accaunts, unlock_ix).await?);
    }

    let sigs = sigs.into_iter().flatten().collect::<Vec<_>>();

    Ok(sigs)
}
