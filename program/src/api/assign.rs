use {
    crate::state::State,
    solana_program::{
        account_info::AccountInfo, entrypoint_deprecated::ProgramResult, msg,
        program::invoke_signed, system_instruction,
    },
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
};

pub fn assign_account<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("assign_account");

    if data.len() <= PUBKEY_BYTES + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (owner_bytes, seed_bytes) = data.split_at(PUBKEY_BYTES);
    let owner = Pubkey::new_from_array(owner_bytes.try_into().unwrap());
    let seed: &[u8] = seed_bytes.try_into().unwrap();

    let state = State::new(program, accounts)?;

    let (key, bump) = Pubkey::find_program_address(&[seed], program);

    let info = state.get(key)?;

    invoke_signed(
        &system_instruction::assign(info.key, &owner),
        &[info.clone()],
        &[&[data, &[bump]]],
    )?;

    Ok(())
}
