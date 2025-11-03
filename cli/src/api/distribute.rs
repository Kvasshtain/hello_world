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

    let from_context = Context::new(context.program_id, context.keypair, context.client).unwrap();
    let to_context = Context::new(context.program_id, &to, context.client).unwrap();

    if internal_transfer(context, amount, mint, to.pubkey()).await.is_err() {
        return Err(anyhow::Error::msg("Internal error"));
    }

    let fut1 = Box::pin(batch(from_context, mint, unfunded));
    let fut2 = Box::pin(batch(to_context, mint, next));

    let res = futures_util::future::join_all(vec![fut1, fut2])
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>();

    res
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Result<Vec<Signature>> {
    //let mut futures: Vec<_> = vec![];
    //let mut accounts = vec![];

    let mut accounts = (0..count)
        .into_iter()
        .map(|_| Keypair::new())
            .collect::<Vec<_>>();

    let acc = &mut accounts; //_!!!!!!!!!!!!

    let futures = acc
        .into_iter()
        .map(|pair| native_transfer(
            &context,
            MIN_BALANCE,
            pair.pubkey(),
        ))
        .collect::<Vec<_>>();

    // for _ in 0..count {
    //     accounts.push(Keypair::new());
    //     futures.push(native_transfer(
    //         &context,
    //         MIN_BALANCE,
    //         accounts[accounts.len() - 1].pubkey(),
    //     ));
    // }

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
    ).await
}
