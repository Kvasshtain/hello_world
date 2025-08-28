use {
    solana_account_info::{next_account_info, AccountInfo},
    solana_msg::msg,
    solana_program::{program::invoke, rent::Rent, sysvar::Sysvar},
    solana_program::{program::invoke_signed, system_instruction},
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction::transfer,
    std::cmp::Ordering::*,
};

pub fn allocate_account(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    seed: &[u8],
    size: u64,
) -> ProgramResult {
    msg!("allocate_account");

    let iter = &mut accounts.iter();

    let signer = next_account_info(iter)?;
    let info = next_account_info(iter)?;
    let system = next_account_info(iter)?;

    // check if account has required length
    if info.data_len() != 0usize {
        return Err(ProgramError::InvalidInstructionData);
    }

    let allocate_ix = system_instruction::allocate(info.key, size);

    let (_new_key, bump) = Pubkey::find_program_address(&[seed], _program_id);

    invoke_signed(
        &allocate_ix,
        &[info.clone(), system.clone()],
        &[&[seed, &[bump]]],
    )?;

    let rent = Rent::get()?.minimum_balance(info.data_len());

    match rent.cmp(&info.lamports()) {
        Greater => {
            let transfer_ix = transfer(signer.key, info.key, rent - info.lamports());
            let infos = vec![signer.clone(), info.clone()];
            invoke(&transfer_ix, &infos)?;
        }
        Less => {
            let refund = info.lamports() - rent;
            **info.try_borrow_mut_lamports()? -= refund;
            **signer.try_borrow_mut_lamports()? += refund;
        }
        _ => {}
    }

    Ok(())
}
