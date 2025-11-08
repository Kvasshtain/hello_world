use {clap::Parser, clap::Subcommand, solana_sdk::pubkey::Pubkey};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(about = "cli application for the hello-world program", long_about = None) ]
pub struct Args {
    /// Hello_world program_id
    #[arg(long)]
    pub program_id: Pubkey,
    /// URL for Solana's JSON RPC: http://localhost:8899
    #[arg(long, default_value = "http://localhost:8899")]
    pub url: String,
    /// Path to signer keypair file
    #[arg(long, default_value = "/home/kvasshtain/.config/solana/id.json")]
    pub keypair_path: String,
    /// Hello_world instruction
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Clone, Debug)]
#[repr(u8)]
pub enum Cmd {
    /// registry a rollup in rome-evm contract
    Create {
        /// Seed for PDA.
        seed: String,
        /// New account size.
        size: u64,
        /// New account owner pubkey.
        owner: Pubkey,
    },
    Resize {
        /// Seed for PDA.
        seed: String,
        /// Account new size.
        size: u64,
    },
    Transfer {
        /// Destination account Id (To which transfer will be done)
        to: Pubkey,
        /// Lamports to send.
        amount: u64,
    },
    TransferFrom {
        /// Seed for PDA.
        seed: String,
        /// Destination account Id (To which transfer will be done)
        to: Pubkey,
        /// Source account Id (From which transfer will be done)
        from: Pubkey,
        /// Lamports to send.
        amount: u64,
    },
    Allocate {
        /// Seed for PDA.
        seed: String,
        /// Account new size.
        size: u64,
    },
    Assign {
        /// Seed for PDA.
        seed: String,
        /// owner account pubkey.
        owner: Pubkey,
    },
    Deposit {
        /// Tokens to send.
        amount: u64,
        /// Mint account pubkey.
        mint: Pubkey,
    },
    InternalTransfer {
        /// Tokens to send.
        amount: u64,
        /// Mint account pubkey.
        mint: Pubkey,
        /// Destination account pubkey.
        to: Pubkey,
    },
    Distribute {
        /// Mint account pubkey.
        mint: Pubkey,
        /// Distributed accounts count
        count: u64,
    },
    FullDistribute {
        /// Mint account pubkey.
        mint: Pubkey,
        /// Distributed accounts count
        count: u64,
    },
}
