use {
    crate::{
        api::{deposit, internal_transfer, native_transfer},
        context::Context,
    },
    anyhow::Result,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
    },
};

const MIN_BALANCE: u64 = 1000000000;
const BUNCH_SIZE: u64 = 10000;

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
        return Err(anyhow::Error::msg("Internal error"));
    }

    println!("depth: {}", depth);

    depth += 1;

    let fut1 = Box::pin(batch(from_context, mint, unfunded, depth));

    depth += 1;

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

async fn distribute_bunch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Result<Vec<Signature>> {
    let mut accounts = (0..count)
        .into_iter()
        .map(|_| Keypair::new())
        .collect::<Vec<_>>();

    let acc = &mut accounts;

    let futures = acc
        .into_iter()
        .map(|pair| native_transfer(
            &context,
            MIN_BALANCE,
            pair.pubkey(),
        ))
        .collect::<Vec<_>>();

    if futures_util::future::join_all(futures).await.iter().any(Result::is_err) {
        return Err(anyhow::Error::msg("Internal error"));
    };

    if deposit(context.clone(), count, mint).await.is_err() {
        return Err(anyhow::Error::msg("Internal error"));
    }

    let to = accounts.pop().unwrap();

    if internal_transfer(context.clone(), count, mint, to.pubkey()).await.is_err() {
        return Err(anyhow::Error::msg("Internal error"));
    }

    batch(
        Context::new(context.program_id, &to, context.client)?,
        mint,
        accounts,
        0
    ).await
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Result<Vec<Signature>> {
    if count <= BUNCH_SIZE {
        return distribute_bunch(context, mint, count).await;
    }

    let fut1: Vec<_> = (0..count / BUNCH_SIZE)
        .into_iter()
        .map(async |_| {
            distribute_bunch(context.clone(), mint, BUNCH_SIZE).await
        })
        .collect();

    let fut2: Vec<_> = std::iter::once(0)
        .map(async |_| {
            distribute_bunch(context.clone(), mint, count - BUNCH_SIZE * (count / BUNCH_SIZE)).await
        })
        .collect();

    Ok(futures_util::future::join_all(fut1).await
        .into_iter()
        .chain(futures_util::future::join_all(fut2).await.into_iter())
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>())
}
