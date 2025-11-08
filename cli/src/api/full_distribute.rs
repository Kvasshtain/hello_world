use {
    crate::{
        api::{internal_transfer, distribute},
        context::Context,
    },
    anyhow::Result,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signature, write_keypair_file},
        signer::Signer,
    },
};
use crate::api::deposit;

pub async fn full_distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Result<Vec<Signature>> {

    if deposit(context.clone(), count, mint).await.is_err() {
        return Err(anyhow::Error::msg("Internal error 1"));
    }

    let genesis = Keypair::new();

    let file_name = format!("key_pairs/recipient{}.json", 0);
    let _ = write_keypair_file(&genesis, file_name);

    let _ = internal_transfer(context.clone(), count, mint, genesis.pubkey()).await;

    let genesis_contest = Context::new(context.program_id, &genesis, context.client)?;

    let res = distribute(genesis_contest, mint, count).await;

    res
}