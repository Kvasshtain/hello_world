//! Program state processor

use {
    crate::api::*,
    solana_account_info::AccountInfo, solana_msg::msg, solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError, solana_pubkey::Pubkey, 
};

// transaction executor
pub fn execute(
    program_id: &Pubkey,  // program_id of contract
    accounts: &[AccountInfo],  // list of transaction accounts
    data: &[u8],    // instruction data
) -> ProgramResult {
    
    // log transaction for debug
    let keys = accounts
        .iter()
        .map(|a| a.key)
        .collect::<Vec<_>>();
    msg!("accounts: {:?}", keys);
    msg!("data: {}", hex::encode(data));

    // parse transaction data
    let (left, right) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    // call instruction handler
    match left {
        0 => create_account(program_id, accounts, right),
        1 => {
            let len = u64::from_le_bytes(right.try_into().unwrap());
            resize_account(program_id, accounts, len as usize)

        }
        _ => Err(ProgramError::InvalidInstructionData)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_create_account() {}
    #[test]
    fn test_resize_account() {}
}
