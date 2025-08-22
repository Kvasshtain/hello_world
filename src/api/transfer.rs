use solana_msg::msg;
use solana_program::system_instruction;
use solana_program_error::ProgramError;
use {
    solana_pubkey::Pubkey,
    solana_account_info::{next_account_info, AccountInfo},
    solana_program::{
        program::invoke,
    },
    solana_program_entrypoint::ProgramResult,
    std::cmp::Ordering::*,
};

// resize solana account data
pub fn transfer_from_to(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64
) -> ProgramResult {
    msg!("transfer_to_account");

    let iter = &mut accounts.iter();
    
    let payer = next_account_info(iter)?;
    let destination = next_account_info(iter)?;
    let system_program_info = next_account_info(iter)?;     // system_program ("11111111111111111111111111111111")

    // if !payer.is_signer {
    //     return Err(ProgramError::MissingRequiredSignature);
    // }

    msg!("Lamports amount: {}", amount);
    
    // Create and invoke the transfer instruction
    let transfer_ix = system_instruction::transfer(
        payer.key,
        destination.key,
        amount,
    );

    invoke(
        &transfer_ix,
        &[
            payer.clone(),
            destination.clone(),
            system_program_info.clone(),
        ],
    )?;

    Ok(())
}