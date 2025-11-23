use {
    crate::{
        accounts::{account_state::AccountState, Data},
        error::Error::CalculationOverflow,
        state::State,
    },
    solana_msg::msg,
    solana_program::{account_info::AccountInfo, entrypoint_deprecated::ProgramResult},
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
    std::mem,
};

pub fn internal_transfer<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("internal_transfer");

    if data.len() < 2 * PUBKEY_BYTES + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (amount_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());

    let (mint_bytes, to_bytes) = rest.split_at(PUBKEY_BYTES);

    let mint_key = Pubkey::try_from(mint_bytes).unwrap();

    let to_key = Pubkey::try_from(to_bytes).unwrap();

    let state = State::new(program, accounts)?;

    let from_pda = state.balance_info(state.signer().key, &mint_key)?;

    let to_pda = state.balance_info(&to_key, &mint_key)?;

    let mut from = AccountState::from_account_mut(from_pda)?;

    from.balance = from
        .balance
        .checked_sub(amount)
        .ok_or(CalculationOverflow)?;

    let mut to = AccountState::from_account_mut(to_pda)?;
    to.balance = to.balance.checked_add(amount).ok_or(CalculationOverflow)?;

    Ok(())
}
