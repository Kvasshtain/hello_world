use {
    solana_account_info::{next_account_info, AccountInfo},
    solana_msg::msg,
    solana_program::{program::invoke_signed, system_instruction},
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
};

pub fn transfer_from(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seed: &[u8],
    amount: u64,
) -> ProgramResult {
    msg!("transfer_from");

    let iter = &mut accounts.iter();

    let payer = next_account_info(iter)?;
    let from = next_account_info(iter)?;
    let to = next_account_info(iter)?;
    let system = next_account_info(iter)?;

    if from.owner == program_id && to.owner != program_id {
        msg!("<<<borrow>>>");
        let (from_key, _bump) = Pubkey::find_program_address(&[seed], program_id);

        if from_key != *from.key {
            return Err(ProgramError::InvalidInstructionData);
        }

        **from.try_borrow_mut_lamports()? -= amount;
        **to.try_borrow_mut_lamports()? += amount;

        return Ok(());
    }

    if from.owner == system.key {
        msg!("<<<transfer>>>");
        let (from_key, bump) = Pubkey::find_program_address(&[seed], program_id);

        if from_key != *from.key {
            return Err(ProgramError::InvalidInstructionData);
        }

        invoke_signed(
            &system_instruction::transfer(from.key, to.key, amount),
            &[payer.clone(), from.clone(), to.clone()],
            &[&[seed, &[bump]]],
        )?;

        return Ok(());
    }

    Err(ProgramError::InvalidInstructionData)
}
