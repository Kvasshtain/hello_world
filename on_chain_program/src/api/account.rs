use {
    crate::api::config::{DATA_SIZE, PROGRAM_WALLET_SEED},
    solana_account_info::AccountInfo,
    solana_program::{program::invoke, program_error::ProgramError, rent::Rent, sysvar::Sysvar},
    solana_program_error::ProgramResult,
    solana_pubkey::Pubkey,
    spl_associated_token_account::tools::account::create_pda_account,
    spl_associated_token_account_client::{
        address::get_associated_token_address_and_bump_seed_internal,
        instruction::create_associated_token_account,
    },
};

pub fn create_program_wallet<'a>(
    payer: &AccountInfo<'a>,
    program_id: &Pubkey,
    program_wallet: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    let (program_wallet_key, bump) =
        Pubkey::find_program_address(&[PROGRAM_WALLET_SEED], program_id);

    if program_wallet_key != *program_wallet.key {
        return Err(ProgramError::InvalidInstructionData);
    }

    if program_wallet.lamports() > 0 && program_wallet.owner != system_program.key {
        return Ok(());
    }

    create_pda_account(
        payer,
        &Rent::get()?,
        DATA_SIZE,
        program_id,
        system_program,
        program_wallet,
        &[PROGRAM_WALLET_SEED, &[bump]],
    )
}

pub fn create_ata_program_wallet<'a>(
    payer: &AccountInfo<'a>,
    program_id: &Pubkey,
    token_program: &AccountInfo<'a>,
    program_wallet: &AccountInfo<'a>,
    ata_program_wallet: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    if ata_program_wallet.lamports() > 0 {
        return Ok(());
    }

    let (program_wallet_key, bump) =
        Pubkey::find_program_address(&[PROGRAM_WALLET_SEED], program_id);

    if program_wallet_key != *program_wallet.key {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (ata_program_wallet_key, _bump) = get_associated_token_address_and_bump_seed_internal(
        program_wallet.key,
        mint.key,
        program_id,
        &spl_token::ID, //token_program.key,
    );

    if ata_program_wallet_key != *ata_program_wallet.key {
        return Err(ProgramError::InvalidInstructionData);
    }

    if ata_program_wallet.owner != system_program.key {
        return Err(ProgramError::InvalidInstructionData);
    }

    let ix =
        create_associated_token_account(&payer.key, &program_wallet.key, &mint.key, &spl_token::ID);

    let infos = [
        payer.clone(),
        ata_program_wallet.clone(),
        program_wallet.clone(),
        mint.clone(),
        system_program.clone(),
        token_program.clone(),
    ];

    invoke(&ix, &infos)
}

pub fn create_user_pda<'a>(
    payer: &AccountInfo<'a>,
    program_id: &Pubkey,
    usr_pda: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
) -> ProgramResult {
    let (usr_pda_key, bump) = Pubkey::find_program_address(&[&payer.key.to_bytes()], program_id);

    if usr_pda_key != *usr_pda.key {
        return Err(ProgramError::InvalidInstructionData);
    }

    if usr_pda.lamports() > 0 && usr_pda.owner != system_program.key {
        return Ok(());
    }

    create_pda_account(
        payer,
        &Rent::get()?,
        DATA_SIZE,
        program_id,
        system_program,
        usr_pda,
        &[PROGRAM_WALLET_SEED, &[bump]],
    )
}
