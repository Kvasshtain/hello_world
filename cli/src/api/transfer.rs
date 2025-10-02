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

pub async fn transfer(program_id: Pubkey,
                      keypair: Keypair,
                      client: &RpcClient,
                      amount: u64,
                      to: String,
) -> Result<Signature> {
    let mut data = vec![2];
    data.extend(amount.to_le_bytes());
    build_tx(
        program_id,
        data,
        &client,
        keypair,
        Pubkey::from_str(to.as_str())?,
    ).await
}