use {
    crate::api::user_data::UserData,
    std::mem::size_of,
};

pub const PROGRAM_WALLET_SEED: &[u8] = "PROGRAM_WALLET_SEED".as_bytes();

pub const DATA_SIZE: usize = size_of::<UserData>();
