use {
    crate::transfer_from_transaction::build_transfer_from_tx,
    anyhow::Result,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        pubkey::Pubkey,
        signature::Signer,
        signature::{
            Keypair,
            Signature,
        },
    },
    std::str::FromStr,
};

pub async fn transfer_from(program_id: Pubkey,
                           keypair: Keypair,
                           client: &RpcClient,
                           amount: u64,
                           seed: String,
                           from: String,
                           to: String,
) -> Result<Signature> {
    let mut data = vec![3];
    data.extend(amount.to_le_bytes());
    let seed = seed;
    data.extend(seed.as_bytes());
    let from = Pubkey::from_str(from.as_str())?;
    build_transfer_from_tx(
        program_id,
        data,
        &client,
        keypair,
        from,
        Pubkey::from_str(to.as_str())?,
    ).await
}