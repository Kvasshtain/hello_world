use std::mem;
use std::os::linux::raw::stat;
use {
    crate::{
        accounts::{account_state::AccountState, Data},
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
use crate::error::Error::AccountLocked;

pub fn lock<'a>(
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

    let balance_pda_from = state.balance_info(&state.signer().key, &mint_key)?;
    let balance_pda_to = state.balance_info(&pubkey, &mint_key)?;

    let mut account_lock_from = AccountLock::from_account_mut(balance_pda_from)?;
    if account_lock_from.get()? {
        return Err(AccountLocked.into());
    }
    
    let mut account_lock_to = AccountLock::from_account_mut(balance_pda_to)?;
    if account_lock_to.get()? {
        return Err(AccountLocked.into());
    }

    let holder = state.holder(uid, &mint_key)?;

    let account_state_from = AccountState::from_account(balance_pda_from)?;
    let account_state_to = AccountState::from_account(balance_pda_to)?;

    HolderData::add_from(holder, account_state_from.balance)?;
    let index = HolderData::add_to(holder, account_state_to.balance)?;

    account_lock_from.lock(holder.key, 0)?;
    account_lock_to.lock(holder.key, index)?;
    
    let mut holder_lock = HolderLock::from_account_mut(holder)?;
    
    holder_lock.update()?;
    
    Ok(())
}
