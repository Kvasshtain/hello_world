use {
    solana_program::{
        account_info::next_account_info, account_info::AccountInfo,
        entrypoint_deprecated::ProgramResult,
    },
    solana_program::{msg, program::invoke_signed, rent::Rent, sysvar::Sysvar},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_pubkey::PUBKEY_BYTES,
    solana_system_interface::instruction,
    std::mem,
};

pub fn create_account(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("create_account");

    if data.len() <= PUBKEY_BYTES + mem::size_of::<u64>() + 1 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (size_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let size = u64::from_le_bytes(size_bytes.try_into().unwrap()) as usize;
    let (owner_bytes, seed_bytes) = rest.split_at(PUBKEY_BYTES);
    let owner = Pubkey::new_from_array(owner_bytes.try_into().unwrap());
    let seed: &[u8] = seed_bytes.try_into().unwrap();

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
