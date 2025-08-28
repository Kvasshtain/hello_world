use std::mem;
use {
    crate::api::{
        allocate::allocate_account, assign::assign_account, transfer::transfer,
        transfer_from::transfer_from, *,
    },
    solana_account_info::AccountInfo,
    solana_msg::msg,
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
};

pub fn execute(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let keys = accounts.iter().map(|a| a.key).collect::<Vec<_>>();
    msg!("accounts: {:?}", keys);
    msg!("data: {}", hex::encode(data));

    let (left, right) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match left {
        0 => create_account(program_id, accounts, right),
        1 => {
            let size = u64::from_le_bytes(right.try_into().unwrap());
            resize_account(program_id, accounts, size as usize)
        }
        2 => {
            let amount = u64::from_le_bytes(right.try_into().unwrap());
            transfer(program_id, accounts, amount)
        }
        3 => {
            if right.len() <= mem::size_of::<u64>() {
                drop(ProgramError::InvalidInstructionData);
            }
            let (amount_bytes, seed_bytes) = right.split_at(mem::size_of::<u64>());
            let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
            let seed: &[u8] = seed_bytes.try_into().unwrap();
            transfer_from(program_id, accounts, seed, amount)
        }
        4 => {
            if right.len() <= mem::size_of::<u64>() {
                drop(ProgramError::InvalidInstructionData);
            }
            let (size_bytes, seed_bytes) = right.split_at(mem::size_of::<u64>());
            let size = u64::from_le_bytes(size_bytes.try_into().unwrap());
            let seed: &[u8] = seed_bytes.try_into().unwrap();
            allocate_account(program_id, accounts, seed, size)
        }
        5 => assign_account(program_id, accounts, right),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create_account() {}
    #[test]
    fn test_resize_account() {}
}
