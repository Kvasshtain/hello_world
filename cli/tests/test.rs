use {
    memo_cli::{api::full_distribute, context::Context},
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

#[rstest]
#[serial_test::serial]
//#[case(500)]
//#[case(5000)]
#[case(50000)]
async fn test(#[case] count: u64) {
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

    let program_id = Pubkey::from_str("Dfjw9nvSTnidg32X8VJNCK3GD1WuQVsz1EhbyrKDwt2j").unwrap();

    let mint_pubkey = mint.pubkey();
    println!("mint_pubkey!!!!! = {}", mint_pubkey);
    //let mint_pubkey = Pubkey::from_str("GPuA19jNYX76DYz9Vdw87xvSqXoKUjo239KvdXAm2c8C").unwrap(); //Delete!!!!!!

    let context = Context::new(program_id, &keypair, &client).unwrap();

    let amount = 5;

    let _result = full_distribute(context, mint_pubkey, count, amount).await;

    let dir = fs::read_dir("./key_pairs").unwrap();

    let mut count: u64 = 0; //Delete!!!!!!

    for entry in dir {
        println!("count = {}", count);

        count = count + 1;

        let entry = entry.unwrap();
        let path = entry.path();

        let keypair: Keypair = read_keypair_file(path).unwrap();
        println!("pubkey!!!!! = {}", keypair.pubkey());

        let context = Context::new(program_id, &keypair, &client).unwrap();

        let balance = Context::get_balance(context.clone(), mint_pubkey)
            .await
            .unwrap();

        println!("balance {}", balance);
        println!("amount {}", amount);

        assert_eq!(balance, amount);
    }
}
