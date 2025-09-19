use {
    solana_account_info::{next_account_info, AccountInfo},
    solana_program::{msg, program::invoke_signed, rent::Rent, sysvar::Sysvar},
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction,
};

pub fn create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    size: usize,
    owner: Pubkey,
    seed: &[u8],
) -> ProgramResult {
    msg!("create_account");

    msg!("owner pubkey: {}", owner.to_string());

    let iter = &mut accounts.iter();

    let signer = next_account_info(iter)?;
    let new = next_account_info(iter)?;
    let _system = next_account_info(iter)?;

    let rent = Rent::get()?.minimum_balance(size);

    let (new_key, bump) = Pubkey::find_program_address(&[seed], program_id);

    if new_key != *new.key {
        return Err(ProgramError::InvalidInstructionData);
    }

    let ix = &instruction::create_account(signer.key, new.key, rent, size as u64, &owner);

    let infos = vec![signer.clone(), new.clone()];

    invoke_signed(&ix, &infos, &[&[seed, &[bump]]])
}
