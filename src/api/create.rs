use {
    solana_pubkey::Pubkey,
    solana_account_info::{next_account_info, AccountInfo},
    solana_system_interface::instruction,
    solana_program_error::ProgramError,
    solana_program::{
        program::invoke_signed,
        rent::Rent, sysvar::Sysvar, msg,
    },
    solana_program_entrypoint::ProgramResult,
};

pub fn create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seed: &[u8]
) -> ProgramResult {
    msg!("create_account");

    let iter = &mut accounts.iter();

    let signer = next_account_info(iter)?;
    let new = next_account_info(iter)?;
    let _system = next_account_info(iter)?;

    let rent = Rent::get()?.minimum_balance(0);

    let (new_key, bump) = Pubkey::find_program_address(&[seed], program_id);

    if new_key != *new.key {
        return Err(ProgramError::InvalidInstructionData)
    }

    let ix = &instruction::create_account(
        signer.key,
        new.key,
        rent,
        0,
        &program_id,
    );

    let infos = vec![signer.clone(), new.clone()];

    invoke_signed(&ix, &infos, &[  &[ seed, &[bump] ]  ])
}