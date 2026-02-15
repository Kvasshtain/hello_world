use std::mem;
use {
    crate::{
        accounts::Data,
        state::State,
    },
    solana_msg::msg,
    solana_program::{account_info::AccountInfo, entrypoint_deprecated::ProgramResult},
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
};
use crate::accounts::account_lock::AccountLock;
use crate::accounts::holder_data::HolderData;
use crate::accounts::holder_lock::HolderLock;
use crate::error::Error;
use crate::error::Error::AccountLocked;

pub fn unlock<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("lock");

    if data.len() < 2 * PUBKEY_BYTES + mem::size_of::<u128>() {
        msg!("Error1");
        return Err(ProgramError::InvalidInstructionData);
    }

    let (mint_bytes, rest) = data.split_at(PUBKEY_BYTES);

    let mint_key = Pubkey::try_from(mint_bytes).unwrap();

    let (pubkey_bytes, rest) = rest.split_at(PUBKEY_BYTES);

    let pubkey = Pubkey::try_from(pubkey_bytes).unwrap();

    let uid = u128::from_le_bytes(rest.try_into().unwrap());

    let state = State::new(program, accounts)?;

    let balance_pda = state.balance_info(&pubkey, &mint_key)?;

    let mut account_lock = AccountLock::from_account_mut(balance_pda)?;

    if !account_lock.get()? {
        return Ok(());
    }

    let holder = state.holder(uid, &mint_key)?;

    if (account_lock.holder != *holder.key) {
        return Err(Error::WrongAccount.into());
    }

    let holder_lock = HolderLock::from_account_mut(holder)?;

    if holder_lock.get()? {
        return Err(AccountLocked.into());
    }

    let account_data = HolderData::get(holder, *balance_pda.key)?;

    let mut dest_data = balance_pda.data.borrow_mut();

    if dest_data.len() != account_data.data.len() {
        return Err(ProgramError::AccountDataTooSmall);
    }

    dest_data.copy_from_slice(&account_data.data);

    account_lock.unlock()?;

    Ok(())
}
