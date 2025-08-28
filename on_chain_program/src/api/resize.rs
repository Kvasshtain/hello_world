use {
    solana_account_info::{next_account_info, AccountInfo},
    solana_msg::msg,
    solana_program::{program::invoke, rent::Rent, sysvar::Sysvar},
    solana_program_entrypoint::ProgramResult,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction::transfer,
    std::cmp::Ordering::*,
};

pub fn resize_account(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    size: usize,
) -> ProgramResult {
    msg!("resize_account");

    let iter = &mut accounts.iter();

    let signer = next_account_info(iter)?;
    let info = next_account_info(iter)?;
    let _system = next_account_info(iter)?;

    if info.data_len() == size {
        return Ok(());
    }

    info.resize(size)?;
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
