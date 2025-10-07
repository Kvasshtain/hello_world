use {
    solana_msg::msg,
    solana_program::{
        account_info::next_account_info, account_info::AccountInfo,
        entrypoint_deprecated::ProgramResult,
    },
    solana_program::{program::invoke, system_instruction},
    solana_pubkey::Pubkey,
};

pub fn transfer(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("transfer");

    let amount = u64::from_le_bytes(data.try_into().unwrap());

    let iter = &mut accounts.iter();

    let payer = next_account_info(iter)?;
    let to = next_account_info(iter)?;
    let system_program_info = next_account_info(iter)?;

    let transfer_ix = system_instruction::transfer(payer.key, to.key, amount);

    invoke(
        &transfer_ix,
        &[payer.clone(), to.clone(), system_program_info.clone()],
    )
}
