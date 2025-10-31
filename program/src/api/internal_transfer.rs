use {
    crate::{
        accounts::{account_state::AccountState, Data},
        error::Error::CalculationOverflow,
        state::State,
    },
    solana_msg::msg,
    solana_program::{
        account_info::AccountInfo, entrypoint_deprecated::ProgramResult, program::invoke_signed,
    },
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

    let mint_key = Pubkey::new_from_array(mint_bytes.try_into().unwrap());

    let to = Pubkey::new_from_array(to_bytes.try_into().unwrap());

    let state = State::new(program, accounts)?;

    msg!("to: {}", to);

    let signer_pda = state.balance_info(state.signer(), &mint_key)?;

    let to_pda = state.balance_info(&to, &mint_key)?;

    let mut account_state = AccountState::from_account_mut(signer_pda)?;

    msg!("amount: {}", amount);

    let b = account_state.balance;

    msg!("account_state.balance: {}", b);

    account_state.balance = account_state
        .balance
        .checked_sub(amount)
        .ok_or(CalculationOverflow)?;

    let mut account_state = AccountState::from_account_mut(to_pda)?;
    account_state.balance = account_state
        .balance
        .checked_add(amount)
        .ok_or(CalculationOverflow)?;

    Ok(())
}
