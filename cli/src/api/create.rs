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

pub async fn create(program_id: Pubkey,
                    keypair: Keypair,
                    client: &RpcClient,
                    seed: String,
                    size: u64,
                    owner_pubkey: String
) -> Result<Signature> {
    let mut data = vec![0];
    data.extend(size.to_le_bytes());
    data.extend(Pubkey::from_str(owner_pubkey.as_str())?.to_bytes());
    let seed = seed;
    data.extend(seed.as_bytes());
    let (new, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
    build_tx(program_id, data, &client, keypair, new).await
}