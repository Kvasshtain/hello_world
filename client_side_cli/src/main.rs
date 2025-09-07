mod instructions;
mod program_option;
mod transaction_data;
mod transactions;
mod transfer_from_transaction;

use {
    crate::program_option::{Args, TransactionType},
    crate::transaction_data::show_tx_data,
    crate::transactions::build_tx,
    crate::transfer_from_transaction::build_transfer_from_tx,
    anyhow::Result,
    clap::Parser,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        pubkey::Pubkey,
        signature::{Signature, read_keypair_file},
    },
    std::{path::Path, str::FromStr},
};

// path to keypair
//const KEYPAIR_PATH: &str = "/home/kvasshtain/.config/solana/id.json";

// solana is running on local machine
//const SOLANA_URL: &str = "http://localhost:8899";

//const PROGRAM_ID: &str = "4fnvoc7wADwtwJ9SRUvL7KpCBTp8qztm5GqjZBFP7GTt";

pub async fn send_tx(args: Args, client: &RpcClient) -> Result<Signature> {
    let tx_sig = read_keypair_file(Path::new(args.keypair_path.as_str())).unwrap();

    // 2: 2nd - account to create
    let program_id: Pubkey = Pubkey::from_str(args.program_pubkey.as_str())?;

    let mut data = vec![args.mode.clone() as u8];

    let sig = match args.mode {
        TransactionType::Create => {
            data.extend(args.size.unwrap().to_le_bytes());
            data.extend(Pubkey::from_str(args.owner_pubkey.unwrap().as_str())?.to_bytes());
            let seed = args.seed.unwrap();
            data.extend(seed.as_bytes());
            let (new, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
            build_tx(program_id, data, &client, tx_sig, new).await?
        }
        TransactionType::Resize => {
            data.extend(args.size.unwrap().to_le_bytes());
            let resized: Pubkey = Pubkey::from_str(args.pda_pubkey.unwrap().as_str())?;
            build_tx(program_id, data, &client, tx_sig, resized).await?
        }
        TransactionType::Transfer => {
            data.extend(args.amount.unwrap().to_le_bytes());
            build_tx(
                program_id,
                data,
                &client,
                tx_sig,
                Pubkey::from_str(args.to.unwrap().as_str())?,
            )
            .await?
        }
        TransactionType::TransferFrom => {
            data.extend(args.amount.unwrap().to_le_bytes());
            let seed = args.seed.unwrap();
            data.extend(seed.as_bytes());
            //let (from, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
            let from = Pubkey::from_str(args.from.unwrap().as_str())?;
            build_transfer_from_tx(
                program_id,
                data,
                &client,
                tx_sig,
                from,
                Pubkey::from_str(args.to.unwrap().as_str())?,
            )
            .await?
        }
        TransactionType::Allocate => {
            data.extend(args.size.unwrap().to_le_bytes());
            let seed = args.seed.unwrap();
            data.extend(seed.as_bytes());
            let (resized, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
            build_tx(program_id, data, &client, tx_sig, resized).await?
        }
        TransactionType::Assign => {
            let seed = args.seed.unwrap();
            data.extend(seed.as_bytes());
            let new: Pubkey = Pubkey::from_str(args.pda_pubkey.unwrap().as_str())?;
            build_tx(program_id, data, &client, tx_sig, new).await?
        }
    };

    println!("job has been done, solana signature: {}", sig);

    Ok(sig)
}

pub async fn execute(args: Args) -> Result<()> {
    let client = RpcClient::new_with_commitment(
        args.solana_url.to_string(),
        CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        },
    );

    let sig = send_tx(args, &client).await?;

    println!("we have done it, solana signature: {}", sig);

    show_tx_data(&client, sig).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    execute(Args::parse()).await?;
    Ok(())
}
