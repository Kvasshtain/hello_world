use {
    anyhow::Result,
    solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcTransactionConfig},
    solana_sdk::{commitment_config::CommitmentConfig, signature::Signature},
    solana_transaction_status_client_types::{
        EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding,
    },
};

pub async fn read_transaction(
    client: &RpcClient,
    sig: &Signature,
) -> Result<EncodedConfirmedTransactionWithStatusMeta> {
    let config = RpcTransactionConfig {
        commitment: CommitmentConfig::confirmed().into(),
        encoding: UiTransactionEncoding::Base64.into(),
        max_supported_transaction_version: Some(0),
    };

    Ok(client.get_transaction_with_config(&sig, config).await?)
}

pub enum SigEnum {
    Fu(Signature),
    Bar(Vec<Signature>),
}

impl From<Signature> for SigEnum {
    fn from(s: Signature) -> Self {
        SigEnum::Fu(s)
    }
}
impl From<Vec<Signature>> for SigEnum {
    fn from(n: Vec<Signature>) -> Self {
        SigEnum::Bar(n)
    }
}

async fn show_tx_log_for_sig(sig: &Signature, client: &RpcClient) -> Result<()> {
    println!("signature: {}", sig);

    let tx_data = read_transaction(client, &sig).await?;

    println!("Transaction data is {:#?}", tx_data);

    Ok(())
}

pub async fn show_tx_log(sig: &SigEnum, client: &RpcClient) -> Result<()> {
    match sig {
        SigEnum::Fu(sig) => show_tx_log_for_sig(sig, client).await,
        SigEnum::Bar(sigs) => {
            let futs = sigs
                .iter()
                .map(|sig| show_tx_log_for_sig(sig, client))
                .collect::<Vec<_>>();

            futures_util::future::join_all(futs)
                .await
                .into_iter()
                .collect::<Result<()>>()
        }
    }
}
