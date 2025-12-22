pub mod api;
pub mod context;
pub mod program_option;
pub mod transaction_log;

use {
    crate::{
        api::{
            allocate, assign, create, create_send_tx, deposit, distribute, full_distribute,
            internal_transfer_ix, native_transfer_from, native_transfer_ix, resize, withdraw,
        },
        context::Context,
        program_option::{Args, Cmd},
        transaction_log::{show_tx_log, SigEnum},
    },
    anyhow::Result,
    clap::Parser,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
    },
    std::path::Path,
};

pub async fn send_tx(args: Args, client: &RpcClient) -> Result<SigEnum> {
    let keypair: Keypair = read_keypair_file(Path::new(args.keypair_path.as_str())).unwrap();

    let program_id: Pubkey = args.program_id;

    let context = Context::new(program_id, &keypair, client)?;

    let result: SigEnum = match args.cmd {
        Cmd::Create {
            seed,
            size,
            owner: owner_pubkey,
        } => create(&context, seed, size, owner_pubkey).await?.into(),
        Cmd::Resize { size, seed } => resize(context, seed, size).await?.into(),
        Cmd::Transfer { amount, to } => {
            let native_transfer_ix = native_transfer_ix(&context, amount, to).await?;
            create_send_tx(&context, &[native_transfer_ix])
                .await?
                .into()
        }
        Cmd::TransferFrom {
            amount,
            seed,
            from,
            to,
        } => native_transfer_from(&context, amount, seed, from, to)
            .await?
            .into(),
        Cmd::Allocate { size, seed } => allocate(context, seed, size).await?.into(),
        Cmd::Assign { seed, owner } => assign(context, seed, owner).await?.into(),
        Cmd::Deposit { amount, mint } => deposit(context, amount, mint).await?.into(),
        Cmd::Withdraw { amount, mint, to } => withdraw(context, amount, mint, to).await?.into(),
        Cmd::InternalTransfer { amount, mint, to } => {
            let internal_transfer_ix = internal_transfer_ix(&context, amount, mint, to).await?;
            create_send_tx(&context, &[internal_transfer_ix])
                .await?
                .into()
        }
        Cmd::Distribute {
            mint,
            count,
            amount,
        } => distribute(context, mint, count, amount).await?.into(),
        Cmd::FullDistribute {
            mint,
            count,
            amount,
        } => full_distribute(context, mint, count, amount).await?.into(),
    };

    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = RpcClient::new_with_commitment(
        args.url.to_string(),
        CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        },
    );

    let result = send_tx(args, &client).await;

    println!("we have done it");

    show_tx_log(&result?, &client).await?;

    Ok(())
}
