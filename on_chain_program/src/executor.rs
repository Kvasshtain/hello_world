use solana_pubkey::PUBKEY_BYTES;
use std::mem;
use {
    crate::api::{
        allocate::allocate_account, assign::assign_account, create_spl::create_spl,
        deposit::deposit, transfer::transfer, transfer_from::transfer_from, *,
    },
    solana_account_info::AccountInfo,
    solana_msg::msg,
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
};

pub fn execute<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &'a [u8],
) -> ProgramResult {
    let keys = accounts.iter().map(|a| a.key).collect::<Vec<_>>();
    msg!("accounts: {:?}", keys);
    msg!("data: {}", hex::encode(data));

    let (left, right) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match left {
        0 => {
            if right.len() <= PUBKEY_BYTES + mem::size_of::<u64>() + 1 {
                return Err(ProgramError::InvalidInstructionData);
            }
            let (size_bytes, rest) = right.split_at(mem::size_of::<u64>());
            let size = u64::from_le_bytes(size_bytes.try_into().unwrap());
            let (owner_bytes, seed_bytes) = rest.split_at(PUBKEY_BYTES);
            let owner = Pubkey::new_from_array(owner_bytes.try_into().unwrap());
            let seed: &[u8] = seed_bytes.try_into().unwrap();
            create_account(program_id, accounts, size as usize, owner, seed)
        }
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
                return Err(ProgramError::InvalidInstructionData);
            }
            let (amount_bytes, seed_bytes) = right.split_at(mem::size_of::<u64>());
            let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
            let seed: &[u8] = seed_bytes.try_into().unwrap();
            transfer_from(program_id, accounts, seed, amount)
        }
        4 => {
            if right.len() <= mem::size_of::<u64>() {
                return Err(ProgramError::InvalidInstructionData);
            }
            let (size_bytes, seed_bytes) = right.split_at(mem::size_of::<u64>());
            let size = u64::from_le_bytes(size_bytes.try_into().unwrap());
            let seed: &[u8] = seed_bytes.try_into().unwrap();
            allocate_account(program_id, accounts, seed, size)
        }
        5 => assign_account(program_id, accounts, right),
        6 => deposit(
            program_id,
            accounts,
            u64::from_le_bytes(right.try_into().unwrap()),
        ),
        //7 => create_spl(accounts),
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
