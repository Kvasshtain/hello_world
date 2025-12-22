use {
    crate::{
        api::{
            create_send_tx::create_send_tx, deposit, distribute,
            internal_transfer_ix::internal_transfer_ix, native_transfer_ix::native_transfer_ix,
            LAMPORTS,
        },
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
    amount: u64,
) -> Result<Vec<Signature>> {
    let _ = deposit(context.clone(), count * amount, mint).await;

    let genesis = Keypair::new();

    let file_name = format!("key_pairs/recipient{}.json", 0);

    let _ = write_keypair_file(&genesis, file_name);

    let native_transfer_ix = native_transfer_ix(&context, LAMPORTS * count, genesis.pubkey()).await;

    println!("LAMPORTS * count {}", LAMPORTS * count);

    let internal_transfer_ix =
        internal_transfer_ix(&context, count * amount, mint, genesis.pubkey()).await;

    println!("count {}", count);

    println!("amount {}", amount);

    println!("count * amount {}", count * amount);

    let _ = create_send_tx(&context, &[native_transfer_ix?, internal_transfer_ix?]).await;

    let genesis_context = Context::new(context.program_id, &genesis, context.client)?;

    distribute(genesis_context, mint, count - 1, amount).await
}
