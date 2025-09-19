use {
    crate::instructions::build_ix,
    anyhow::Result,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signature, Signer},
        transaction::Transaction,
    },
};

pub async fn build_deposit_tx(
    program_id: Pubkey,
    data: Vec<u8>,
    client: &RpcClient,
    payer: Keypair,
    ata_user_wallet: Pubkey,
    user_pda: Pubkey,
    program_wallet: Pubkey,
    ata_program_wallet: Pubkey,
    token_program: Pubkey,
    spl_associated_token_account: Pubkey,
    mint: Pubkey,
) -> Result<Signature> {
    let payer_key = payer.pubkey();

    let ix = build_ix(
        program_id,
        &data.as_slice(),
        payer_key,
        &[
            &ata_user_wallet,
            &user_pda,
            &program_wallet,
            &ata_program_wallet,
            &token_program,
            &spl_associated_token_account,
            &mint,
        ],
    );

    let block_hash = client.get_latest_blockhash().await?;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer_key));

    tx.sign(&[&payer], block_hash);

    Ok(client.send_and_confirm_transaction(&tx).await?)
}
