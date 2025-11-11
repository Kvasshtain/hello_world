use {
    crate::{
        accounts::{account_state::AccountState, Data},
        api::{internal_transfer, native_transfer},
        context::Context,
    },
    anyhow::Result,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{write_keypair_file, Keypair, Signature},
        signer::Signer,
    },
    std::cell::RefCell,
};

pub const MIN_BALANCE: u64 = 1000000000;
const CHUNK_SIZE: usize = 10000;

pub async fn batch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    mut unfunded: Vec<Keypair>,
) -> Result<Vec<Signature>> {
    if unfunded.is_empty() {
        return Ok(vec![]);
    }

    let to = unfunded.pop().unwrap();
    let at = unfunded.len() / 2;

    let next = unfunded.split_off(at);

    let amount = (next.len() + 1) as u64;

    let from_context = Context::new(context.program_id, context.keypair, context.client)?;
    let to_context = Context::new(context.program_id, &to, context.client)?;

    let result = internal_transfer(context, amount, mint, to.pubkey()).await;

    if result.is_err() {
        return Err(anyhow::Error::msg("Internal transfer error"));
    }

    let fut1 = Box::pin(batch(from_context, mint, unfunded));

    let fut2 = Box::pin(batch(to_context, mint, next));

    let results = futures_util::future::join_all(vec![fut1, fut2])
        .await
        .into_iter()
        .chain(std::iter::once(result))
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
) -> Result<Vec<Signature>> {
    let rec = &mut recipients;

    let futures = rec
        .into_iter()
        .map(|pair| native_transfer(&context, MIN_BALANCE, pair.pubkey()))
        .collect::<Vec<_>>();

    let results = futures_util::future::join_all(futures).await;

    if results.iter().any(Result::is_err) {
        return Err(anyhow::Error::msg("Native transfer error"));
    };

    let count = recipients.len() as u64;

    let to = recipients.pop().unwrap();

    let result = internal_transfer(context.clone(), count, mint, to.pubkey()).await;

    if result.is_err() {
        return Err(anyhow::Error::msg("Internal transfer error"));
    }

    let mut sigs = result?;

    let mut batch_sigs = batch(
        Context::new(context.program_id, &to, context.client)?,
        mint,
        recipients,
    )
    .await?;

    sigs.append(&mut batch_sigs);

    Ok(sigs)
}

fn into_chunks<T>(mut vec: Vec<T>, size: usize) -> Vec<Vec<T>> {
    if size == 0 {
        panic!("chuck size must be greater than zero");
    }

    let mut chunks = vec![];

    while !vec.is_empty() {
        let x = vec.drain(..size.min(vec.len())).collect::<Vec<_>>();
        chunks.push(x);
    }

    chunks
}

async fn get_balance<'a>(context: Context<'a>, mint: Pubkey) -> Result<u64> {
    let (pubkey, _bump) = Pubkey::find_program_address(
        &[&context.keypair.pubkey().to_bytes(), &mint.to_bytes()],
        &context.program_id,
    );

    let data = context.client.get_account_data(&pubkey).await?;

    let ref_cell = RefCell::new(data.as_slice());

    let account_state = AccountState::from_arr(ref_cell.borrow()).await?;

    Ok(account_state.balance)
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Result<Vec<Signature>> {
    let balance = get_balance(context.clone(), mint).await?;

    if balance != count {
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
            .map(async |chunk| distribute_chunk(context.clone(), mint, chunk).await),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>>>()?
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    Ok(sigs)
}
