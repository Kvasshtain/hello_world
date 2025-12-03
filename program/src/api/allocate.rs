use {
    crate::state::State,
    solana_program::{
        account_info::AccountInfo, entrypoint_deprecated::ProgramResult, msg, program::invoke,
        program::invoke_signed, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
    },
    solana_program_error::ProgramError,
    solana_system_interface::instruction::transfer,
    std::cmp::Ordering::*,
    std::mem,
};

pub fn allocate_account<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("allocate_account");

    if data.len() <= mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (size_bytes, seed) = data.split_at(mem::size_of::<u64>());
    let size = u64::from_le_bytes(size_bytes.try_into().unwrap());

    let state = State::new(program, accounts)?;

    let (key, bump) = Pubkey::find_program_address(&[seed], program);

    let ix = system_instruction::allocate(&key, size);

    invoke_signed(&ix, &state.infos(&ix)?, &[&[seed, &[bump]]])?;

    let signer = state.signer();
    let info = state.get(key)?;

    let rent = Rent::get()?.minimum_balance(info.data_len());

    match rent.cmp(&info.lamports()) {
        Greater => {
            let transfer_ix = transfer(signer.key, info.key, rent - info.lamports());
            let infos = vec![signer.clone(), info.clone()];
            invoke(&transfer_ix, &infos)?;
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
