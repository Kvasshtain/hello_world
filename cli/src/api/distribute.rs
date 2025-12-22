use {
    crate::{
        api::{create_send_tx, internal_transfer_ix, native_transfer_ix},
        context::Context,
    },
    anyhow::Result,
    futures_util::future::join_all,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{read_keypair_file, write_keypair_file, Keypair, Signature},
        signer::Signer,
    },
    std::{path::Path, sync::Arc},
    tokio::{sync::Semaphore, task::JoinHandle},
};

pub const LAMPORTS: u64 = 1000000000;
const CHUNK_SIZE: usize = 300;//1000;

pub async fn batch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    mut unfunded: Vec<Keypair>,
    amount: u64,
    depth: u64,
) -> Result<Vec<Signature>> {
    if unfunded.is_empty() {
        return Ok(vec![]);
    }

    let to = unfunded.pop().unwrap();
    let at = unfunded.len() / 2;

    let next = unfunded.split_off(at);

    let new_amount = ((next.len() + 1) as u64) * amount;
    let new_lamports = ((next.len() + 1) as u64) * LAMPORTS;

    let from_context = Context::new(context.program_id, context.keypair, context.client)?;
    let to_context = Context::new(context.program_id, &to, context.client)?;

    let native_transfer_ix = native_transfer_ix(&context, new_lamports, to.pubkey()).await;
    let internal_transfer_ix = internal_transfer_ix(&context, new_amount, mint, to.pubkey()).await;

    let result =
        vec![create_send_tx(&context, &[native_transfer_ix?, internal_transfer_ix?]).await?];

    let fut1 = Box::pin(batch(from_context, mint, unfunded, amount, depth + 1));

    let fut2 = Box::pin(batch(to_context, mint, next, amount, depth + 1));

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

async fn save_recipients(
    tasks: usize,
    recipients: Vec<Keypair>,
) -> (Vec<JoinHandle<()>>, Vec<String>) {
    let semaphore = Arc::new(Semaphore::new(tasks));

    let mut jh = vec![];

    let mut i = 0;

    let mut paths: Vec<String> = vec![];

    for recipient in recipients {
        let semaphore = Arc::clone(&semaphore);
        let permit = semaphore.acquire_owned().await.unwrap();

        i = i + 1;

        let path = format!("./key_pairs/recipient{}.json", i);
        paths.push(path.clone());

        let file_name = path;

        let handle = tokio::spawn(async move {
            let _ = write_keypair_file(&recipient, file_name);
            drop(permit);
        });

        jh.push(handle);
    }
    (jh, paths)
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
    amount: u64,
) -> Result<Vec<Signature>> {
    let balance = Context::get_balance(context.clone(), mint).await?;

    if balance != (count + 1) * amount {
        return Err(anyhow::Error::msg("Insufficient balance"));
    }

    let recipients = (0..count)
        .into_iter()
        .map(|_i| Keypair::new())
        .collect::<Vec<_>>();

    let (tasks, paths) = save_recipients(16, recipients).await;

    let _ = join_all(tasks).await;

    let mut recipients = vec![];

    for path_str in paths {
        let path = Path::new(&path_str);

        let keypair: Keypair = read_keypair_file(path).unwrap();
        recipients.push(keypair);
    }

    let mut sigs = vec![];

    for i in into_chunks(recipients, CHUNK_SIZE) {
        sigs.push(batch(context.clone(), mint, i, amount, 0).await?)
    }

    let sigs = sigs.into_iter().flatten().collect::<Vec<_>>();

    Ok(sigs)
}
