use solana_program_error::ProgramError;
use {
    solana_pubkey::Pubkey,
    solana_account_info::{
        next_account_info,
        AccountInfo
    },
    solana_program_entrypoint::ProgramResult,
    solana_msg::msg,
    solana_program::{
        program::invoke_signed,
        system_instruction,
    },
};

pub fn assign_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seed: &[u8],
) -> ProgramResult {
    msg!("assign_account");

    let iter = &mut accounts.iter();

    let signer = next_account_info(iter)?;
    let info = next_account_info(iter)?;
    let _system = next_account_info(iter)?;

    msg!("program_id: {}", program_id);

    let (new_key, bump) = Pubkey::find_program_address(&[seed], program_id); //_system.key);

    if new_key != *info.key {

        return Err(ProgramError::InvalidInstructionData)
    }

    msg!("seed: {}", String::from_utf8(Vec::from(seed)).unwrap());
    msg!("new_key: {}", new_key);
    msg!("*info.key: {}", *info.key);
    msg!("*_system.key: {}", *_system.key);

    invoke_signed(
        &system_instruction::assign(
            info.key,
            &program_id,
        ),
        &[info.clone()],
        //&[&[signer.key.as_ref(), seed, &[bump]]],
        &[&[seed, &[bump]]],
    )?;

    Ok(())
}