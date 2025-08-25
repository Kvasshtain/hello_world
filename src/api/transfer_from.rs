use solana_program::program::invoke_signed;
use solana_program_error::ProgramError;
use {
    solana_msg::msg,
    solana_pubkey::Pubkey,
    solana_account_info::{next_account_info, AccountInfo},
    solana_program_entrypoint::ProgramResult,
};

pub fn transfer_from(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seed: &[u8],
    amount: u64
) -> ProgramResult {
    msg!("transfer_from");

    let iter = &mut accounts.iter();

    let _payer = next_account_info(iter)?;
    let from = next_account_info(iter)?;
    let to = next_account_info(iter)?;
    let _system_program_info = next_account_info(iter)?;

    let (from_key, bump) = Pubkey::find_program_address(&[seed], program_id);

    if from_key != *from.key {
        return Err(ProgramError::InvalidInstructionData)
    }

    **from.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;

    Ok(())
}