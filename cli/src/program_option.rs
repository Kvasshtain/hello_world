use clap::{Subcommand};
use solana_sdk::pubkey::Pubkey;
pub(crate) use {clap::Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(about = "cli application for the hello-world program", long_about = None) ]
pub struct Args {
    /// Hello_world program_id
    #[arg(long)]
    pub program_id: String,
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
        owner_pubkey: String,
    } = 0,
    Resize {
        /// Account new size.
        size: u64,
        /// PDA-account pubkey.
        pda_pubkey: String,
    } = 1,
    Transfer {
        /// Destination account Id (To which transfer will be done)
        to: String,
        /// Lamports to send.
        amount: u64,
    } = 2,
    TransferFrom {
        /// Seed for PDA.
        seed: String,
        /// Destination account Id (To which transfer will be done)
        to: String,
        /// Source account Id (From which transfer will be done)
        from: String,
        /// Lamports to send.
        amount: u64,
    } = 3,
    Allocate {
        /// Seed for PDA.
        seed: String,
        /// Account new size.
        size: u64,
    } = 4,
    Assign {
        /// Seed for PDA.
        seed: String,
        /// PDA-account pubkey.
        pda_pubkey: String,
    } = 5,
    Deposit {
        /// Tokens to send.
        amount: u64,
        /// Mint account pubkey.
        mint: String,
    } = 6,
}
