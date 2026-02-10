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
use crate::accounts::holder_data::HolderData;
use crate::accounts::holder_lock::HolderLock;

pub fn multiple_transfer<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("multiple_transfer");

    if data.len() < PUBKEY_BYTES + mem::size_of::<u128>() + mem::size_of::<u64>() {
        msg!("Error1");
        return Err(ProgramError::InvalidInstructionData);
    }

    let (mint_bytes, rest) = data.split_at(PUBKEY_BYTES);
    let mint_key = Pubkey::try_from(mint_bytes).unwrap();

    let (uid_bytes, rest) = rest.split_at(mem::size_of::<u64>());
    let uid = u128::from_le_bytes(uid_bytes.try_into().unwrap());

    let amount = u64::from_le_bytes(rest.try_into().unwrap());

    let state = State::new(program, accounts)?;

    let holder = state.holder(uid, &mint_key)?;

    let mut holder_data = HolderData::from_account_mut(holder)?;

    let from_value = holder_data[0];

    let to_len = holder_data.len() as u64 - 1;

    holder_data[0] = from_value.checked_sub(to_len * amount)
        .ok_or(CalculationOverflow)?;
    
    for i in 1..to_len {
        holder_data[i as usize] = holder_data[i as usize] + amount;
    }

    let mut holder_lock = HolderLock::from_account_mut(holder)?;

    holder_lock.unlock();

    Ok(())
}
