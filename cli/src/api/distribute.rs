use {
    crate::{
        api::{internal_transfer, native_transfer},
        context::Context,
    },
    anyhow::Result,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{write_keypair_file, Keypair, Signature},
        signer::Signer,
    },
};

pub const LAMPORTS: u64 = 1000000000;
const CHUNK_SIZE: usize = 10000;

pub async fn batch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    mut unfunded: Vec<Keypair>,
    amount: u64,
) -> Result<Vec<Signature>> {
    if unfunded.is_empty() {
        return Ok(vec![]);
    }

    let to = unfunded.pop().unwrap();
    let at = unfunded.len() / 2;

    let next = unfunded.split_off(at);

    let new_amount = ((next.len() + 1) as u64) * amount;

    let from_context = Context::new(context.program_id, context.keypair, context.client)?;
    let to_context = Context::new(context.program_id, &to, context.client)?;

    let result = vec![internal_transfer(context, new_amount, mint, to.pubkey()).await?];

    let fut1 = Box::pin(batch(from_context, mint, unfunded, amount));

    let fut2 = Box::pin(batch(to_context, mint, next, amount));

    let results = futures_util::future::join_all(vec![fut1, fut2])
        .await
        .into_iter()
        .chain(std::iter::once(Ok(result)))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(results)
}

async fn distribute_chunk<'a>(
    context: Context<'a>,
    mint: Pubkey,
    mut recipients: Vec<Keypair>,
    amount: u64,
) -> Result<Vec<Signature>> {
    let rec = &mut recipients;

    let futures = rec
        .into_iter()
        .map(|pair| native_transfer(&context, LAMPORTS, pair.pubkey()))
        .collect::<Vec<_>>();

    futures_util::future::join_all(futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    let count = recipients.len() as u64;

    let to = recipients.pop().unwrap();

    let result = internal_transfer(context.clone(), count * amount, mint, to.pubkey()).await?;

    let mut sigs = vec![result];

    let mut batch_sigs = batch(
        Context::new(context.program_id, &to, context.client)?,
        mint,
        recipients,
        amount,
    )
    .await?;

    sigs.append(&mut batch_sigs);

    Ok(sigs)
}

fn into_chunks<T>(mut vec: Vec<T>, size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        panic!("chunk size must be greater than zero");
    }

    let mut chunks = vec![];

    while !vec.is_empty() {
        let x = vec.drain(..size.min(vec.len())).collect::<Vec<_>>();
        chunks.push(x);
    }

    chunks
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
    amount: u64,
) -> Result<Vec<Signature>> {
    let balance = Context::get_balance(context.clone(), mint).await?;

    if balance != count * amount {
        return Err(anyhow::Error::msg("Insufficient balance"));
    }

    let recipients = (1..count)
        .into_iter()
        .map(|i| {
            let keypair = Keypair::new();
            let file_name = format!("key_pairs/recipient{}.json", i);
            let _ = write_keypair_file(&keypair, file_name);
            keypair
        })
        .collect::<Vec<_>>();

    let sigs = futures_util::future::join_all(
        into_chunks(recipients, CHUNK_SIZE)
            .into_iter()
            .map(async |chunk| distribute_chunk(context.clone(), mint, chunk, amount).await),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>>>()?
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    Ok(sigs)
}
