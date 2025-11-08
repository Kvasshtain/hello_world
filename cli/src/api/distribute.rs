use solana_client::nonblocking::rpc_client::RpcClient;
use {
    crate::{
        api::{deposit, internal_transfer, native_transfer},
        context::Context,
        accounts::{account_state::AccountState, Data,}
    },
    anyhow::Result,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signature, write_keypair_file},
        signer::Signer,
    },
    std::cell::RefCell,
};
use crate::transaction_log::read_transaction;

const MIN_BALANCE: u64 = 1000000000;
const CHUNK_SIZE: usize = 10000;

pub async fn batch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    mut unfunded: Vec<Keypair>,
    mut depth: u64,
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
        return Err(anyhow::Error::msg("Internal error 0"));
    }

    println!("depth: {}", depth);

    depth += 1;

    let fut1 = Box::pin(batch(from_context, mint, unfunded, depth));

    let fut2 = Box::pin(batch(to_context, mint, next, depth));

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
        .map(|pair| native_transfer(
                &context,
                MIN_BALANCE,
                pair.pubkey(),
                ))
        .collect::<Vec<_>>();

    let results = futures_util::future::join_all(futures).await;

    //println!("&&&&&&&&&&&&&&&results.len(): {}", results.len());

    //let res = results[0];

    //println!("&&&&&&&&&&&&&&&results.len(): {:#?}", read_transaction(context.client, &results[0].unwrap()[0]).await.unwrap());

    // results
    //     .into_iter()
    //     .map(|res| {
    //         println!("&&&&&&&&&&&&&&&");
    //         res
    //             .into_iter()
    //             .map(|v| {
    //                 v
    //                     .into_iter()
    //                     .map(async |sig| {
    //                         let data = read_transaction(context.client, &sig).await.unwrap();
    //                         println!("*****************data: {:#?}", data)
    //                     })
    //             })
    //     });

    if results.iter().any(Result::is_err) {
        return Err(anyhow::Error::msg("Internal error 2"));
    };

    let count = recipients.len() as u64;

    let to = recipients.pop().unwrap();

    let result = internal_transfer(context.clone(), count, mint, to.pubkey()).await;

    if result.is_err() {
        return Err(anyhow::Error::msg("Internal error 3"));
    }

    let res = batch(
        Context::new(context.program_id, &to, context.client)?,
        mint,
        recipients,
        0
    ).await;

    let mut sigs = res?;

    sigs.append(&mut result?);

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

async fn get_balance<'a>(
    context: Context<'a>,
    mint: Pubkey,
) -> Result<u64> {
    let (pubkey, _bump) = Pubkey::find_program_address(&[&context.keypair.pubkey().to_bytes(), &mint.to_bytes()], &context.program_id);

    println!("????????/pubkey: {}", pubkey);

    let data = context.client.get_account_data(&pubkey).await?;

    let data = data.as_slice();

    let ref_cell = RefCell::new(data);

    let data = ref_cell.borrow();

    let account_state = AccountState::from_arr(data).await?;

    Ok(account_state.balance)
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Result<Vec<Signature>> {

    println!("context.keypair.pubkey(): {}", context.keypair.pubkey());

    let balance = get_balance(context.clone(), mint).await?;




    println!("!!!!!balance: {}", balance);

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

    let chunks = into_chunks(recipients, CHUNK_SIZE);

    let mut results = vec![];

    for chunk in chunks {
        results.push(distribute_chunk(context.clone(), mint, chunk).await);
    }

    let res = results
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    Ok(res)
}
