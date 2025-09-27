use {crate::accounts::account_state::AccountState, std::mem::size_of};

pub const WALLET_SEED: &[u8] = "PROGRAM_WALLET_SEED".as_bytes();

pub const DATA_SIZE: usize = size_of::<AccountState>();

pub const MINT_SIZE: usize = 82;
