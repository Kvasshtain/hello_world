mod deposit_transactions;
mod instructions;
mod program_option;
mod transaction_data;
mod transactions;
mod transfer_from_transaction;
mod api;

use {
    crate::{
        api::{
            create,
            resize,
            transfer,
            allocate,
            assign,
            deposit,
            transfer_from,
        },
        program_option::{Args, Cmd},
        transaction_data::show_tx_data,
    },
    anyhow::Result,
    clap::Parser,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{
            CommitmentConfig,
            CommitmentLevel,
        },
        pubkey::Pubkey,
        signature::Signer,
        signature::{
            Keypair,
            Signature,
            read_keypair_file
        },
    },
    std::{
        path::Path,
        str::FromStr,
    },
};

pub async fn send_tx(args: Args, client: &RpcClient) -> Result<Signature> {
    let keypair: Keypair = read_keypair_file(Path::new(args.keypair_path.as_str())).unwrap();

    let program_id: Pubkey = args.program_id;

    let sig = match args.cmd {
        Cmd::Create {
            seed,
            size,
            owner_pubkey,
        } => create(program_id, keypair, client, seed, size, owner_pubkey).await?,
        Cmd::Resize {
            size,
            pda_pubkey,
        } => resize(program_id, keypair, client, pda_pubkey, size).await?,
        Cmd::Transfer {
            amount,
            to,
        } => {
            transfer(program_id, keypair, client, amount, to).await?
        }
        Cmd::TransferFrom {
            amount,
            seed,
            from,
            to,
        } => {
            transfer_from(program_id, keypair, client, amount, seed, from, to).await?
        }
        Cmd::Allocate {
            size,
            seed,
        } => {
            allocate(program_id, keypair, client, seed, size,).await?
        }
        Cmd::Assign {
            seed,
            pda_pubkey,
        } => {
            assign(program_id, keypair, client, seed, pda_pubkey,).await?
        }
        Cmd::Deposit {
            amount,
            mint,
        } => {
            deposit(program_id, keypair, client, amount, mint).await?
        }
    };

    println!("job has been done, solana signature: {}", sig);

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
