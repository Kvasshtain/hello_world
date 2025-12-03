use {
    crate::state::State,
    solana_program::{
        account_info::AccountInfo, entrypoint_deprecated::ProgramResult, msg,
        program::invoke_signed, rent::Rent, sysvar::Sysvar,
    },
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
    solana_system_interface::instruction,
    std::mem,
};

pub fn create_account<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("create_account");

    if data.len() <= PUBKEY_BYTES + mem::size_of::<u64>() + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (size_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let size = u64::from_le_bytes(size_bytes.try_into().unwrap()) as usize;
    let (owner_bytes, seed) = rest.split_at(PUBKEY_BYTES);
    let owner = Pubkey::try_from(owner_bytes).unwrap();

    let state = State::new(program, accounts)?;

    let rent = Rent::get()?.minimum_balance(size);

    let (new_key, bump) = Pubkey::find_program_address(&[seed], program);

    let ix = &instruction::create_account(state.signer().key, &new_key, rent, size as u64, &owner);

    invoke_signed(&ix, &state.infos(&ix)?, &[&[seed, &[bump]]])
}
