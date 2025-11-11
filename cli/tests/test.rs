use {
    memo_cli::{
        accounts::{account_state::AccountState, Data},
        api::full_distribute,
        context::Context,
        transaction_log::show_tx_log,
    },
    rstest::*,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    std::{cell::RefCell, fs, path::Path, str::FromStr},
};

async fn get_balance<'a>(context: Context<'a>, mint: Pubkey) -> anyhow::Result<u64> {
    let (pubkey, _bump) = Pubkey::find_program_address(
        &[&context.keypair.pubkey().to_bytes(), &mint.to_bytes()],
        &context.program_id,
    );

    let data = context.client.get_account_data(&pubkey).await?;

    let data = data.as_slice();

    let ref_cell = RefCell::new(data);

    let data = ref_cell.borrow();

    let account_state = AccountState::from_arr(data).await?;

    Ok(account_state.balance)
}

#[rstest(count, case::coun_1(4))]
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

    let result = full_distribute(context, mint, count).await;

    let _ = show_tx_log(&client, result).await;

    let dir = fs::read_dir("./key_pairs").unwrap();

    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();

        let keypair: Keypair = read_keypair_file(path).unwrap();

        let context = Context::new(program_id, &keypair, &client).unwrap();

        let balance = get_balance(context.clone(), mint).await.unwrap();

        assert_eq!(balance, 1);
    }
}
