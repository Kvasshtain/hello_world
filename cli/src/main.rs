mod api;
mod context;
mod program_option;
mod transaction_data;

use {
    crate::{
        api::{allocate, assign, create, deposit, resize, transfer, transfer_from},
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

pub async fn send_tx(args: Args, client: &RpcClient) -> Result<Signature> {
    let keypair: Keypair = read_keypair_file(Path::new(args.keypair_path.as_str())).unwrap();

    let program_id: Pubkey = args.program_id;

    let state = Context::new(program_id, keypair, client)?;

    let sig = match args.cmd {
        Cmd::Create {
            seed,
            size,
            owner: owner_pubkey,
        } => create(state, seed, size, owner_pubkey).await?,
        Cmd::Resize { size, pda: pda } => resize(state, pda, size).await?,
        Cmd::Transfer { amount, to } => transfer(state, amount, to).await?,
        Cmd::TransferFrom {
            amount,
            seed,
            from,
            to,
        } => transfer_from(state, amount, seed, from, to).await?,
        Cmd::Allocate { size, seed } => allocate(state, seed, size).await?,
        Cmd::Assign { seed, owner } => assign(state, seed, owner).await?,
        Cmd::Deposit { amount, mint } => deposit(state, amount, mint).await?,
    };

    println!("signature: {}", sig);

    Ok(sig)
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

    let sig = send_tx(args, &client).await?;

    println!("we have done it, solana signature: {}", sig);

    show_tx_data(&client, sig).await
}
