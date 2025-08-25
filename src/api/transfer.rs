use {
    solana_msg::msg,
    solana_pubkey::Pubkey,
    solana_account_info::{next_account_info, AccountInfo},
    solana_program::{
        program::invoke,
        system_instruction,
    },
    solana_program_entrypoint::ProgramResult,
};

pub fn transfer(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64
) -> ProgramResult {
    msg!("transfer");

    let iter = &mut accounts.iter();
    
    let payer = next_account_info(iter)?;
    let to = next_account_info(iter)?;
    let system_program_info = next_account_info(iter)?;

    msg!("Lamports amount: {}", amount);

    let transfer_ix = system_instruction::transfer(
        payer.key,
        to.key,
        amount,
    );

    invoke(
        &transfer_ix,
        &[
            payer.clone(),
            to.clone(),
            system_program_info.clone(),
        ],
    )
}