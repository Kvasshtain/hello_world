use {
    crate::api::{
        allocate::allocate_account,
        assign::assign_account,
        deposit::deposit,
        transfer::transfer,
        transfer_from::transfer_from,
        *,
    },
    solana_account_info::AccountInfo,
    solana_msg::msg,
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
};

pub fn execute<'a>(
    program_id: &'a Pubkey,
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
        0 => create_account(program_id, accounts, right),
        1 => resize_account(program_id, accounts, right),
        2 => {
            let amount = u64::from_le_bytes(right.try_into().unwrap());
            transfer(program_id, accounts, right)
        }
        3 => transfer_from(program_id, accounts, right),
        4 => allocate_account(program_id, accounts, right),
        5 => assign_account(program_id, accounts, right),
        6 => deposit(program_id, accounts, right),
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
