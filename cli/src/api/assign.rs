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
    std::str::FromStr,
};

pub async fn assign(program_id: Pubkey,
                    keypair: Keypair,
                    client: &RpcClient,
                    seed: String,
                    pda_pubkey: String,
) -> Result<Signature> {
    let mut data = vec![5];
    let seed = seed;
    data.extend(seed.as_bytes());
    let new: Pubkey = Pubkey::from_str(pda_pubkey.as_str())?;
    build_tx(program_id, data, &client, keypair, new).await
}