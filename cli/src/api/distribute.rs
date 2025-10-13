use {
    crate::{context::Context, api::{internal_transfer, transfer, deposit}},
    solana_sdk::{
        pubkey::Pubkey,
        signer::Signer,
        signature::{Signature, Keypair}
    },
};

const MIN_BALANCE: u64 = 1000000000;

pub async fn batch<'a>(
    context: Context<'a>,
    mint: Pubkey,
    mut unfunded: Vec<Keypair>,
) -> anyhow::Result<Signature> {
    let mut ret_value: anyhow::Result<Signature> = Err(anyhow::anyhow!("ERROR"));

    if unfunded.is_empty() {
        return ret_value;
    }

    let to = unfunded.pop().unwrap();
    let at = unfunded.len() / 2;

    let next = unfunded.split_off(at);

    let amount = (next.len() + 1) as u64;

    let from_context = Context::new(context.program_id, context.keypair, context.client)?;
    let to_context = Context::new(context.program_id, &to, context.client)?;

    ret_value = internal_transfer(context, amount, mint, to.pubkey()).await;

    let fut1 = Box::pin(batch(from_context, mint, unfunded));
    let fut2 = Box::pin(batch(to_context, mint, next));

    futures_util::future::join_all(vec![fut1, fut2]).await;

    ret_value
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> anyhow::Result<Signature> {
    let mut futures: Vec<_> = vec![];

    let mut accounts = vec![];
    for _ in 0..count {
        accounts.push(Keypair::new());
        futures.push(transfer(&context, MIN_BALANCE, accounts[accounts.len() - 1].pubkey()));
    }

    let results = futures_util::future::join_all(futures).await;

    for result in results {
        let _ = result;
    }

    let _ = deposit(context.clone(), count, mint).await;

    let to = accounts.pop().unwrap();

    let _ = internal_transfer(context.clone(), count, mint, to.pubkey()).await;

    batch(Context::new(context.program_id, &to, context.client)?, mint, accounts).await
}