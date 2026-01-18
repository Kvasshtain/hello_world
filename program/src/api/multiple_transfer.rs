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

pub fn multiple_transfer<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("internal_transfer");

    if data.len() < PUBKEY_BYTES + mem::size_of::<u64>() + mem::size_of::<usize>() {
        msg!("Error1");
        return Err(ProgramError::InvalidInstructionData);
    }

    let (amount_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());

    let (mint_bytes, rest) = rest.split_at(PUBKEY_BYTES);
    let mint_key = Pubkey::try_from(mint_bytes).unwrap();

    let (tos_len_bytes, rest) = rest.split_at(mem::size_of::<usize>());
    let tos_len = usize::from_le_bytes(tos_len_bytes.try_into().unwrap());

    if rest.len() < tos_len * PUBKEY_BYTES {
        msg!("Error2");
        return Err(ProgramError::InvalidInstructionData);
    }

    let mut tos = vec![];

    for i in 0..tos_len {
        let tos_bytes = &rest[i * PUBKEY_BYTES .. (i+1) * PUBKEY_BYTES];
        tos.push(Pubkey::try_from(tos_bytes).unwrap());
    }

    let state = State::new(program, accounts)?;

    let from_pda = state.balance_info(state.signer().key, &mint_key)?;

    let mut from = AccountState::from_account_mut(from_pda)?;

    for to_key in &tos {

        msg!("FOR");

        let to_pda = state.balance_info(&to_key, &mint_key)?;

        msg!("1!!!!");

        let b = from.balance;

        msg!("from.balance = {}", b);

        from.balance = from
            .balance
            .checked_sub(amount)
            .ok_or(CalculationOverflow)?;

        msg!("2!!!!");

        let mut to = AccountState::from_account_mut(to_pda)?;

        msg!("3!!!!");

        to.balance = to.balance.checked_add(amount).ok_or(CalculationOverflow)?;

        msg!("4!!!!");
    }

    Ok(())
}
