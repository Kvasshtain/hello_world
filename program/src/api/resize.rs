use {
    crate::state::State,
    solana_msg::msg,
    solana_program::{
        account_info::AccountInfo, entrypoint_deprecated::ProgramResult, program::invoke,
        rent::Rent, sysvar::Sysvar,
    },
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction::transfer,
    std::{cmp::Ordering::*, mem},
};

pub fn resize_account<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("resize_account");

    if data.len() <= mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (size_bytes, seed) = data.split_at(mem::size_of::<u64>());
    let size = usize::from_le_bytes(size_bytes.try_into().unwrap());

    let state = State::new(program, accounts)?;

    let signer = state.signer();

    let (key, _bump) = Pubkey::find_program_address(&[seed], program);
    let info = state.get(key)?;

    if info.data_len() == size {
        return Ok(());
    }

    info.realloc(size, false)?;
    let rent = Rent::get()?.minimum_balance(info.data_len());

    match rent.cmp(&info.lamports()) {
        Greater => {
            let ix = transfer(signer.key, info.key, rent - info.lamports());
            let infos = vec![signer.clone(), info.clone()];
            invoke(&ix, &infos)?;
        }
        Less => {
            let refund = info.lamports() - rent;
            **info.try_borrow_mut_lamports()? -= refund;
            **signer.try_borrow_mut_lamports()? += refund;
        }
        _ => {}
    }

    Ok(())
}
