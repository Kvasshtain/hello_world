use {
    crate::{
        api::{deposit, distribute, internal_transfer, native_transfer, json_string, LAMPORTS},
        context::Context,
    },
    anyhow::Result,
    futures_util::AsyncWriteExt,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    async_std::fs::{OpenOptions, File},
};

pub async fn full_distribute<'a>(
    context: Context<'a>,
    mint: Pubkey,
    count: u64,
    amount: u64,
) -> Result<Vec<Signature>> {
    let _ = deposit(context.clone(), count * amount, mint).await;

    let genesis = Keypair::new();

    let file_name = "key_pairs/recipients.json";

    let text_to_append = json_string(&genesis)?;

    tokio::fs::File::create(file_name).await?;

    let mut file: File = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .await?;

    file.write_all(text_to_append.as_bytes()).await?;

    let _ = native_transfer(&context, LAMPORTS * count, genesis.pubkey()).await;

    let _ = internal_transfer(context.clone(), count * amount, mint, genesis.pubkey()).await;

    let genesis_context = Context::new(context.program_id, &genesis, context.client)?;

    distribute(genesis_context, mint, count, amount, file).await
}
