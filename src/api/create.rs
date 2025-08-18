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

// create new solana account
pub fn create_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seed: &[u8]
) -> ProgramResult {
    msg!("create_account");

    let iter = &mut accounts.iter();

    let signer = next_account_info(iter)?;  // first account - transaction signer
    let new = next_account_info(iter)?; // 2nd - account to create
    let _system = next_account_info(iter)?;     // system_program ("11111111111111111111111111111111")

    // rent exempt  
    // https://solana.com/docs/core/accounts#rent
    let rent = Rent::get()?.minimum_balance(0);

    // calculate address of new account
    // https://solana.com/docs/core/pda
    let (new_key, bump) = Pubkey::find_program_address(&[seed], program_id);

    // calculated key must match the corresponding account in the transaction
    if new_key != *new.key {
        return Err(ProgramError::InvalidInstructionData)
    }

    // instruction of system_program to create new account
    let ix = &instruction::create_account(
        signer.key,
        new.key,
        rent,
        0,
        &program_id, // owner
    );

    let infos = vec![signer.clone(), new.clone()];
    
    // cross-program invocation  
    // https://solana.com/docs/core/cpi
    invoke_signed(&ix, &infos, &[  &[ seed, &[bump] ]  ])?;

    Ok(())
}