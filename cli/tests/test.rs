use {
    memo_cli::{api::full_distribute, context::Context},
    rstest::*,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::{CommitmentConfig, CommitmentLevel},
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair},
    },
    std::{fs::File, path::Path, str::FromStr, io::{BufRead, BufReader}},
};

/// The length of a ed25519 `SecretKey`, in bytes.
pub const SECRET_KEY_LENGTH: usize = 32;

/// The length of an ed25519 `PublicKey`, in bytes.
pub const PUBLIC_KEY_LENGTH: usize = 32;

/// The length of an ed25519 `Keypair`, in bytes.
pub const KEYPAIR_LENGTH: usize = SECRET_KEY_LENGTH + PUBLIC_KEY_LENGTH;

fn from_str(string: String) -> anyhow::Result<Keypair> {
    let trimmed = string.trim();

    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Input must be a JSON array",
        )
            .into());
    }

    let contents = &trimmed[1..trimmed.len() - 1];
    let elements_vec: Vec<&str> = contents.split(',').map(|s| s.trim()).collect();
    let len = elements_vec.len();

    let elements: [&str; KEYPAIR_LENGTH] =
        elements_vec.try_into().map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Expected {} elements, found {}",
                    KEYPAIR_LENGTH,
                    len
                ),
            )
        })?;
    let mut out = [0u8; KEYPAIR_LENGTH];
    for (idx, element) in elements.into_iter().enumerate() {
        let parsed: u8 = element.parse()?;
        out[idx] = parsed;
    }
    Keypair::from_bytes(&out)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()).into())
}

#[rstest(count, case::coun_1(1000))]
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

    let mint = Pubkey::from_str("4wiyrhQdxsq2vbKn7QdFbRYMwAV2i5aeRCZZM9C3znsX").unwrap();

    let context = Context::new(program_id, &keypair, &client).unwrap();

    let amount = 6;

    let _result = full_distribute(context, mint, count, amount).await;

    let file_name = "key_pairs/recipients.json";

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("line{}", line);

        let keypair: Keypair = from_str(line).unwrap();

        let context = Context::new(program_id, &keypair, &client).unwrap();

        let balance = Context::get_balance(context.clone(), mint).await.unwrap();

        assert_eq!(balance, amount);
        println!("balance{}", balance);
    }
}
