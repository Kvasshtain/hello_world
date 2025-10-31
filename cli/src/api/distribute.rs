use {
    crate::{
        api::{deposit, internal_transfer, transfer},
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
) -> Vec<Result<Signature>> {
    let mut ret_value: Vec<Result<Signature>> = vec![];

    if unfunded.is_empty() {
        return ret_value;
    }

    let to = unfunded.pop().unwrap();
    let at = unfunded.len() / 2;

    let next = unfunded.split_off(at);

    let amount = (next.len() + 1) as u64;

    let from_context = Context::new(context.program_id, context.keypair, context.client).unwrap();
    let to_context = Context::new(context.program_id, &to, context.client).unwrap();

    ret_value.append(&mut internal_transfer(context, amount, mint, to.pubkey()).await);

    let fut1 = Box::pin(batch(from_context, mint, unfunded));
    let fut2 = Box::pin(batch(to_context, mint, next));

    let results = futures_util::future::join_all(vec![fut1, fut2]).await;

    for mut result in results {
        ret_value.append(&mut result);
    }

    ret_value
}

pub async fn distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Vec<Result<Signature>> {
    let mut ret_value: Vec<Result<Signature>> = vec![];

    let mut futures: Vec<_> = vec![];

    let mut accounts = vec![];
    for _ in 0..count {
        accounts.push(Keypair::new());
        futures.push(transfer(
            &context,
            MIN_BALANCE,
            accounts[accounts.len() - 1].pubkey(),
        ));
    }

    let results = futures_util::future::join_all(futures).await;

    for mut result in results {
        ret_value.append(&mut result);
    }

    ret_value.append(&mut deposit(context.clone(), count, mint).await);

    let to = accounts.pop().unwrap();

    ret_value.append(&mut internal_transfer(context.clone(), count, mint, to.pubkey()).await);

    ret_value.append(
        &mut batch(
            Context::new(context.program_id, &to, context.client).unwrap(),
            mint,
            accounts,
        )
        .await,
    );

    ret_value
}
