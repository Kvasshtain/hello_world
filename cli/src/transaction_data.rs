use {
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig},
    solana_sdk::{commitment_config::CommitmentConfig, signature::Signature},
    solana_transaction_status_client_types::{
        EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding,
    },
};

pub async fn read_transaction(
    client: &RpcClient,
    sig: Signature,
) -> anyhow::Result<EncodedConfirmedTransactionWithStatusMeta> {
    let config = RpcTransactionConfig {
        commitment: CommitmentConfig::confirmed().into(),
        encoding: UiTransactionEncoding::Base64.into(),
        max_supported_transaction_version: Some(0),
    };

    Ok(client.get_transaction_with_config(&sig, config).await?)
}

pub async fn show_tx_data(client: &RpcClient, sig: Signature) -> anyhow::Result<()> {
    let tx_data = read_transaction(client, sig).await?;

    println!("Transaction data is {:#?}", tx_data);

    Ok(())
}