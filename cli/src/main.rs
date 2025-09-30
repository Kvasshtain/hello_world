mod create_spl_transaction;
mod deposit_transactions;
mod instructions;
mod program_option;
mod transaction_data;
mod transactions;
mod transfer_from_transaction;

const PROGRAM_WALLET_SEED: &[u8] = "PROGRAM_WALLET_SEED".as_bytes();

use {
    crate::{
        deposit_transactions::build_deposit_tx,
        program_option::{Args, Cmd},
        transaction_data::show_tx_data,
        transactions::build_tx,
        transfer_from_transaction::build_transfer_from_tx,
    },
    anyhow::Result,
    clap::Parser,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        pubkey::Pubkey,
        signature::Signer,
        signature::{Keypair, Signature, read_keypair_file},
    },
    spl_associated_token_account_client::{
        address::get_associated_token_address_and_bump_seed_internal,
        instruction::create_associated_token_account,
    },
    std::{path::Path, str::FromStr},
};

pub async fn send_tx(args: Args, client: &RpcClient) -> Result<Signature> {
    let tx_sig: Keypair = read_keypair_file(Path::new(args.keypair_path.as_str())).unwrap();

    // 2: 2nd - account to create
    let program_id: Pubkey = Pubkey::from_str(args.program_id.as_str())?;

    let mut data: Vec<u8>;

    let sig = match args.cmd {
        Cmd::Create {
            seed,
            size,
            owner_pubkey,
        } => {
            data = vec![0];
            data.extend(size.to_le_bytes());
            data.extend(Pubkey::from_str(owner_pubkey.as_str())?.to_bytes());
            let seed = seed;
            data.extend(seed.as_bytes());
            let (new, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
            build_tx(program_id, data, &client, tx_sig, new).await?
        }
        Cmd::Resize {
            size,
            pda_pubkey,
        } => {
            data = vec![1];
            data.extend(size.to_le_bytes());
            let resized: Pubkey = Pubkey::from_str(pda_pubkey.as_str())?;
            build_tx(program_id, data, &client, tx_sig, resized).await?
        }
        Cmd::Transfer {
            amount,
            to,
        } => {
            data = vec![2];
            data.extend(amount.to_le_bytes());
            build_tx(
                program_id,
                data,
                &client,
                tx_sig,
                Pubkey::from_str(to.as_str())?,
            )
            .await?
        }
        Cmd::TransferFrom {
            amount,
            seed,
            from,
            to,
        } => {
            data = vec![3];
            data.extend(amount.to_le_bytes());
            let seed = seed;
            data.extend(seed.as_bytes());
            //let (from, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
            let from = Pubkey::from_str(from.as_str())?;
            build_transfer_from_tx(
                program_id,
                data,
                &client,
                tx_sig,
                from,
                Pubkey::from_str(to.as_str())?,
            )
            .await?
        }
        Cmd::Allocate {
            size,
            seed,
        } => {
            data = vec![4];
            data.extend(size.to_le_bytes());
            let seed = seed;
            data.extend(seed.as_bytes());
            let (resized, _bump) = Pubkey::find_program_address(&[&*seed.as_bytes()], &program_id);
            build_tx(program_id, data, &client, tx_sig, resized).await?
        }
        Cmd::Assign {
            seed,
            pda_pubkey,
        } => {
            data = vec![5];
            let seed = seed;
            data.extend(seed.as_bytes());
            let new: Pubkey = Pubkey::from_str(pda_pubkey.as_str())?;
            build_tx(program_id, data, &client, tx_sig, new).await?
        }
        Cmd::Deposit {
            amount,
            mint,
        } => {
            data = vec![6];
            data.extend(amount.to_le_bytes());
            let (usr_pda_key, _bump) =
                Pubkey::find_program_address(&[&tx_sig.pubkey().to_bytes()], &program_id);
            let mint = Pubkey::from_str(mint.as_str())?;
            data.extend(mint.to_bytes());
            let (program_wallet_key, _bump) =
                Pubkey::find_program_address(&[PROGRAM_WALLET_SEED], &program_id);
            let (ata_user_wallet, _bump) =
                get_associated_token_address_and_bump_seed_internal(
                    &tx_sig.pubkey(),
                    &mint,
                    &spl_associated_token_account::ID,
                    &spl_token::ID,
                );
            let (ata_program_wallet_key, _bump) =
                get_associated_token_address_and_bump_seed_internal(
                    &program_wallet_key,
                    &mint,
                    &spl_associated_token_account::ID,
                    &spl_token::ID,
                );
            build_deposit_tx(
                program_id,
                data,
                &client,
                tx_sig,
                ata_user_wallet,
                usr_pda_key,
                program_wallet_key,
                ata_program_wallet_key,
                spl_token::ID,
                spl_associated_token_account::ID,
                mint,
            )
            .await?
        } 
        // TransactionType::CreateSpl => {
        //     build_create_spl_tx(program_id, data, &client, tx_sig, ).await?
        // }
    };

    println!("job has been done, solana signature: {}", sig);

    Ok(sig)
}

pub async fn execute(args: Args) -> Result<()> {
    let client = RpcClient::new_with_commitment(
        args.url.to_string(),
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
