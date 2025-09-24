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

pub async fn build_transfer_from_tx(
    program_id: Pubkey,
    data: Vec<u8>,
    client: &RpcClient,
    payer: Keypair,
    from: Pubkey,
    to: Pubkey,
) -> Result<Signature> {
    let payer_key = payer.pubkey();

    let ix = build_ix(program_id, &data.as_slice(), payer_key, &[&from, &to]);

    let blockhash = client.get_latest_blockhash().await?;

    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer_key));

    tx.sign(&[&payer], blockhash);

    Ok(client.send_and_confirm_transaction(&tx).await?)
}
