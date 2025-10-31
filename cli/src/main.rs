mod api;
mod context;
mod program_option;
mod transaction_data;

use {
    crate::{
        api::{
            allocate, assign, create, deposit, distribute, internal_transfer, resize, transfer,
            transfer_from,
        },
        context::Context,
        program_option::{Args, Cmd},
        transaction_data::show_tx_data,
    },
    anyhow::Result,
    clap::Parser,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signature},
    },
    std::{path::Path, str::FromStr},
};

pub async fn send_tx(args: Args, client: &RpcClient) -> Vec<Result<Signature>> {
    let keypair: Keypair = read_keypair_file(Path::new(args.keypair_path.as_str())).unwrap();

    let program_id: Pubkey = args.program_id;

    let context = Context::new(program_id, &keypair, client).unwrap();

    let sigs = match args.cmd {
        Cmd::Create {
            seed,
            size,
            owner: owner_pubkey,
        } => create(&context, seed, size, owner_pubkey).await,
        Cmd::Resize { size, seed } => resize(context, seed, size).await,
        Cmd::Transfer { amount, to } => transfer(&context, amount, to).await,
        Cmd::TransferFrom {
            amount,
            seed,
            from,
            to,
        } => transfer_from(&context, amount, seed, from, to).await,
        Cmd::Allocate { size, seed } => allocate(context, seed, size).await,
        Cmd::Assign { seed, owner } => assign(context, seed, owner).await,
        Cmd::Deposit { amount, mint } => deposit(context, amount, mint).await,
        Cmd::InternalTransfer { amount, mint, to } => {
            internal_transfer(context, amount, mint, to).await
        }
        Cmd::Distribute { mint, count } => distribute(context, mint, count).await,
    };

    sigs
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

    let sigs = send_tx(args, &client).await;

    println!("we have done it");

    show_tx_data(&client, sigs).await?;

    Ok(())
}
