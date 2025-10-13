use {
    crate::state::State,
    solana_msg::msg,
    solana_program::{account_info::AccountInfo, entrypoint_deprecated::ProgramResult},
    solana_program::{program::invoke, system_instruction},
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
    std::mem,
};

pub fn native_transfer<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("transfer");

    if data.len() < PUBKEY_BYTES + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (to_bytes, amount_bytes) = data.split_at(PUBKEY_BYTES);
    let to = Pubkey::new_from_array(to_bytes.try_into().unwrap());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());

    let state = State::new(program, accounts)?;

    let signer = state.signer_info()?;

    let ix = system_instruction::transfer(signer.key, &to, amount);

    invoke(&ix, &state.infos(&ix)?)
}
