use {
    crate::state::State,
    solana_msg::msg,
    solana_program::{account_info::AccountInfo, entrypoint_deprecated::ProgramResult},
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
    std::mem,
};
use crate::accounts::holder_data::HolderData;

pub fn cast_data_slice<T>(data: &[u8]) -> &T {
    //assert_eq!(align_of::<T>(), 1);
    //assert_eq!(data.len(), size_of::<T>());

    unsafe { &*data.as_ptr().cast::<T>() }
}

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

    let balance_pda_from = state.balance_info(&state.signer().key, &mint_key)?;

    let account_from = HolderData::get(holder, *balance_pda_from.key)?;

    let holder_map = HolderData::map(holder)?;

    let holder_map_count = holder_map.map.len() as u32;

    let to_len = holder_map_count as u64 - 1;

    HolderData::update_balance(holder, to_len * amount, &account_from, *balance_pda_from.key)?;

    for (balance_pda_to_key, account_to) in holder_map.map.iter().filter(|(&k, _)| k != *balance_pda_from.key) {
        HolderData::update_balance(holder, amount, account_to, *balance_pda_to_key)?;
    }

    Ok(())
}
