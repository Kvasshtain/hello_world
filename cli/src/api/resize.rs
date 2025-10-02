use {
    crate::transactions::build_tx,
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

pub async fn resize(program_id: Pubkey,
                    keypair: Keypair,
                    client: &RpcClient,
                    pda_pubkey: String,
                    size: u64,
) -> Result<Signature> {
    let mut data = vec![1];
    data.extend(size.to_le_bytes());
    let resized: Pubkey = Pubkey::from_str(pda_pubkey.as_str())?;
    build_tx(program_id, data, &client, keypair, resized).await
}