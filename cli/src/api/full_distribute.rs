use {
    crate::{
        api::{deposit, distribute, internal_transfer, native_transfer, MIN_BALANCE},
        context::Context,
    },
    anyhow::Result,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{write_keypair_file, Keypair, Signature},
        signer::Signer,
    },
};

pub async fn full_distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
) -> Result<Vec<Signature>> {
    if deposit(context.clone(), count, mint).await.is_err() {
        return Err(anyhow::Error::msg("Deposit error"));
    }

    let genesis = Keypair::new();

    let file_name = format!("key_pairs/recipient{}.json", 0);
    let _ = write_keypair_file(&genesis, file_name);

    let _ = native_transfer(&context, MIN_BALANCE * count, genesis.pubkey()).await;

    let _ = internal_transfer(context.clone(), count, mint, genesis.pubkey()).await;

    let genesis_context = Context::new(context.program_id, &genesis, context.client)?;

    distribute(genesis_context, mint, count).await
}
