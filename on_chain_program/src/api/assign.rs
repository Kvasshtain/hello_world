use {
    solana_account_info::{next_account_info, AccountInfo},
    solana_msg::msg,
    solana_program::{program::invoke_signed, system_instruction},
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
};

pub fn assign_account(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("assign_account");

    let iter = &mut accounts.iter();

    let _signer = next_account_info(iter)?;
    let info = next_account_info(iter)?;
    let _system = next_account_info(iter)?;

    msg!("program_id: {}", program_id);

    let (new_key, bump) = Pubkey::find_program_address(&[data], program_id);

    if new_key != *info.key {
        return Err(ProgramError::InvalidInstructionData);
    }

    msg!("seed: {}", String::from_utf8(Vec::from(data)).unwrap());
    msg!("new_key: {}", new_key);
    msg!("*info.key: {}", *info.key);
    msg!("*_system.key: {}", *_system.key);

    invoke_signed(
        &system_instruction::assign(info.key, &program_id),
        &[info.clone()],
        &[&[data, &[bump]]],
    )?;

    Ok(())
}
