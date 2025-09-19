mod transaction_type;

use solana_sdk::pubkey::Pubkey;
pub(crate) use {clap::Parser, transaction_type::TransactionType};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(about = "cli application for the hello-world program", long_about = None) ]
pub struct Args {
    /// Solana URL (for example http://localhost:8899)
    #[arg(long, default_value = "http://localhost:8899")]
    pub solana_url: String,
    /// Path to signer keypair file
    #[arg(long, default_value = "/home/kvasshtain/.config/solana/id.json")]
    pub keypair_path: String,
    #[arg(long)]
    /// Pubkey of smart contract
    pub program_pubkey: String,
    /// Program mode (create - Create account, resize - Resize account, transfer - Send lamports from account to account, transfer-from - Send lamports from PDA-account to account, allocate - Allocate account space, assign - Change account owner).
    #[arg(long)]
    pub mode: TransactionType,
    /// Seed for PDA. If not set will be asked
    #[arg(long)]
    #[arg(long, required_if_eq("mode", "create"))]
    #[arg(long, required_if_eq("mode", "transfer-from"))]
    #[arg(long, required_if_eq("mode", "allocate"))]
    #[arg(long, required_if_eq("mode", "assign"))]
    pub seed: Option<String>,
    /// New account size (ignored if mode = 0).
    #[arg(long, required_if_eq("mode", "create"))]
    #[arg(long, required_if_eq("mode", "resize"))]
    #[arg(long, required_if_eq("mode", "allocate"))]
    pub size: Option<u64>,
    /// New account owner pubkey.
    #[arg(long, required_if_eq("mode", "create"))]
    pub owner_pubkey: Option<String>,
    /// Destination account Id (To which transfer will be done)
    #[arg(long, required_if_eq("mode", "transfer"))]
    #[arg(long, required_if_eq("mode", "transfer-from"))]
    pub to: Option<String>,
    /// Source account Id (From which transfer will be done)
    #[arg(long, required_if_eq("mode", "transfer-from"))]
    pub from: Option<String>,
    /// Lamports to send.
    #[arg(long, required_if_eq("mode", "transfer"))]
    #[arg(long, required_if_eq("mode", "transfer-from"))]
    #[arg(long, required_if_eq("mode", "deposit"))]
    pub amount: Option<u64>,
    /// PDA-account pubkey.
    #[arg(long, required_if_eq("mode", "resize"))]
    #[arg(long, required_if_eq("mode", "assign"))]
    pub pda_pubkey: Option<String>,

    /// User ATA-account pubkey.
    #[arg(long, required_if_eq("mode", "deposit"))]
    pub ata_user_wallet: Option<String>,
    /// Mint account pubkey.
    #[arg(long, required_if_eq("mode", "deposit"))]
    pub mint: Option<String>,
}
