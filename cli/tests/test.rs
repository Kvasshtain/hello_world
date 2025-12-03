use {
    memo_cli::{api::full_distribute, context::Context},
    rstest::*,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
    },
    std::{fs, path::Path, str::FromStr},
};

#[rstest(count, case::coun_1(900))]
async fn test(count: u64) {
    let url = "http://localhost:8899";

    let client = RpcClient::new_with_commitment(
        url.to_string(),
        CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        },
    );

    let keypair_path = "/home/kvasshtain/.config/solana/id.json";

    let keypair: Keypair = read_keypair_file(Path::new(keypair_path)).unwrap();

    let program_id = Pubkey::from_str("Dfjw9nvSTnidg32X8VJNCK3GD1WuQVsz1EhbyrKDwt2j").unwrap();

    let mint = Pubkey::from_str("9Q3MAeXatgD3KhPsQ793hXCqbsDT5dWgQE3VndknpZqN").unwrap();

    let context = Context::new(program_id, &keypair, &client).unwrap();

    let amount = 5;

    let _result = full_distribute(context, mint, count, amount).await;

    let dir = fs::read_dir("./key_pairs").unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();

        let keypair: Keypair = read_keypair_file(path).unwrap();

        let context = Context::new(program_id, &keypair, &client).unwrap();

        let balance = Context::get_balance(context.clone(), mint).await.unwrap();

        assert_eq!(balance, amount);
    }
}
