use {
    crate::{
        transactions::build_tx,
    },
    anyhow::Result,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        pubkey::Pubkey,
        signature::Signer,
        signature::{
            Keypair,
            Signature
        },
    },
};

pub async fn allocate(program_id: Pubkey,
                      keypair: Keypair,
                      client: &RpcClient,
                      seed: String,
                      size: u64,
) -> Result<Signature> {
    let mut data = vec![4];
    data.extend(size.to_le_bytes());
    let seed = seed;
    data.extend(seed.as_bytes());
    let (resized, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
    build_tx(program_id, data, &client, keypair, resized).await
}