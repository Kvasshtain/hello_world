use std::io;
use std::sync::Arc;
use futures_util::future::join_all;
use solana_sdk::signature::write_keypair_file;
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use {
    memo_cli::{api::distribute, context::Context},
    rstest::*,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        program_pack::Pack,
        pubkey::Pubkey,
        signature::Signer,
        signature::{read_keypair_file, Keypair},
        system_instruction::create_account,
        transaction::Transaction,
    },
    spl_associated_token_account::{
        get_associated_token_address, instruction::create_associated_token_account,
    },
    spl_token::{
        instruction::{initialize_mint, mint_to},
        state::Mint,
    },
    std::{fs, path::Path, str::FromStr},
};
use memo_cli::api::{deposit, multiple_transfer_ix};

async fn arrange(client: &RpcClient, keypair: &Keypair) -> Keypair {
    let latest_blockhash = client.get_latest_blockhash().await.unwrap();

    let mint = Keypair::new();

    let space = Mint::LEN;
    let rent = client
        .get_minimum_balance_for_rent_exemption(space)
        .await
        .unwrap();

    let create_account_instruction = create_account(
        &keypair.pubkey(),
        &mint.pubkey(),
        rent,
        space as u64,
        &spl_token::ID,
    );

    let decimals: u8 = 0;

    let initialize_mint_instruction = initialize_mint(
        &spl_token::ID,
        &mint.pubkey(),
        &keypair.pubkey(),
        Some(&keypair.pubkey()),
        decimals,
    )
    .unwrap();

    let associated_token_account = get_associated_token_address(&keypair.pubkey(), &mint.pubkey());

    let create_ata_instruction = create_associated_token_account(
        &keypair.pubkey(),
        &keypair.pubkey(),
        &mint.pubkey(),
        &spl_token::ID,
    );

    let amount = 1_000_000_000;

    let mint_to_instruction = mint_to(
        &spl_token::ID,
        &mint.pubkey(),
        &associated_token_account,
        &keypair.pubkey(),
        &[&keypair.pubkey()],
        amount,
    )
    .unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account_instruction,
            initialize_mint_instruction,
            create_ata_instruction,
            mint_to_instruction,
        ],
        Some(&keypair.pubkey()),
        &[&keypair, &mint],
        latest_blockhash,
    );

    client
        .send_and_confirm_transaction(&transaction)
        .await
        .unwrap();

    mint
}

// #[rstest]
// #[serial_test::serial]
// #[case(50)]
// #[case(500)]
// #[case(5000)]
// async fn test(#[case] count: u64) {
//     let url = "http://localhost:8899";
//
//     let client = RpcClient::new_with_commitment(
//         url.to_string(),
//         CommitmentConfig {
//             commitment: CommitmentLevel::Confirmed,
//         },
//     );
//
//     let keypair_path = "/home/kvasshtain/.config/solana/id.json";
//
//     let keypair: Keypair = read_keypair_file(Path::new(keypair_path)).unwrap();
//
//     let mint = arrange(&client, &keypair).await;
//
//     let program_id = Pubkey::from_str("Dfjw9nvSTnidg32X8VJNCK3GD1WuQVsz1EhbyrKDwt2j").unwrap();
//
//     let mint_pubkey = mint.pubkey();
//
//     let context = Context::new(program_id, &keypair, &client).unwrap();
//
//     let amount = 5;
//
//     distribute(context, mint_pubkey, count, amount).await.unwrap();
//
//     let dir = fs::read_dir("./key_pairs").unwrap();
//
//     for entry in dir {
//         let entry = entry.unwrap();
//         let path = entry.path();
//
//         let keypair: Keypair = read_keypair_file(path).unwrap();
//
//         let context = Context::new(program_id, &keypair, &client).unwrap();
//
//         let balance = Context::get_balance(context.clone(), mint_pubkey)
//             .await
//             .unwrap();
//
//         assert_eq!(balance, amount);
//     }
// }

async fn save_recipients(
    tos_dir_path: &str,
    tasks: usize,
    recipients: Vec<Keypair>,
) -> Vec<JoinHandle<()>>{
    let semaphore = Arc::new(Semaphore::new(tasks));

    let mut jh = vec![];

    let mut i = 0;

    for recipient in recipients {
        let semaphore = Arc::clone(&semaphore);
        let permit = semaphore.acquire_owned().await.unwrap();

        i = i + 1;

        let file_name = format!("/recipient{}.json", i);

        let path = tos_dir_path.to_string() + &*file_name;

        let handle = tokio::spawn(async move {
            let _ = write_keypair_file(&recipient, path);
            drop(permit);
        });

        jh.push(handle);
    }

    jh
}

fn clean_directory(path_str: &str) -> io::Result<()> {
    let path = Path::new(path_str);

    if !path.exists() || !path.is_dir(){
        return Ok(())
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry.file_type()?.is_dir() {
            fs::remove_dir_all(&entry_path)?;
        } else {
            fs::remove_file(&entry_path)?;
        }
    }

    Ok(())
}

async fn tos(count: u64, tos_dir_path: &str) {
    let recipients = (0..count - 1)
        .into_iter()
        .map(|_i| Keypair::new())
        .collect::<Vec<_>>();

    let tasks = save_recipients(tos_dir_path, 16, recipients).await;

    let _ = join_all(tasks).await;
}

#[rstest]
#[serial_test::serial]
#[case(15)]
async fn test2(#[case] count: u64) {
    let url = "http://localhost:8899";

    let client = RpcClient::new_with_commitment(
        url.to_string(),
        CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        },
    );

    let keypair_path = "/home/kvasshtain/.config/solana/id.json";

    let keypair: Keypair = read_keypair_file(Path::new(keypair_path)).unwrap();

    let mint = arrange(&client, &keypair).await;

    let tos_dir_path = "./key_pairs";

    clean_directory(tos_dir_path).unwrap();

    tos(count, tos_dir_path).await;

    let program_id = Pubkey::from_str("Dfjw9nvSTnidg32X8VJNCK3GD1WuQVsz1EhbyrKDwt2j").unwrap();

    let mint_pubkey = mint.pubkey();

    let context = Context::new(program_id, &keypair, &client).unwrap();

    let amount = 5;

    let _ = deposit(context.clone(), count * amount, mint_pubkey).await;

    let multiple_transfer_ix = multiple_transfer_ix(&context, amount, mint_pubkey, tos_dir_path.parse().unwrap()).await.unwrap();

    context.client.send_and_confirm_transaction(&context.compose_tx(&[multiple_transfer_ix]).await.unwrap())
        .await
        .unwrap();

    let dir = fs::read_dir(tos_dir_path).unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();

        let keypair: Keypair = read_keypair_file(path).unwrap();

        let context = Context::new(program_id, &keypair, &client).unwrap();

        let balance = Context::get_balance(context.clone(), mint_pubkey)
            .await
            .unwrap();

        assert_eq!(balance, amount);
    }
}
