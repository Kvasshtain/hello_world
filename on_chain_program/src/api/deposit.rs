use {
    crate::api::error::Error::CalculationOverflow,
    crate::api::user_data::UserData,
    crate::api::Data,
    crate::api::{
        account::{create_ata_program_wallet, create_program_wallet, create_user_pda},
        error::Error,
    },
    solana_account_info::{next_account_info, AccountInfo},
    solana_program::program::invoke_signed,
    solana_program_entrypoint::ProgramResult,
    solana_pubkey::Pubkey,
    solana_msg::msg,
    spl_token::instruction::transfer,
};

pub type Result<T> = std::result::Result<T, Error>;

pub fn deposit<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    amount: u64,
) -> ProgramResult {
    msg!("!!!!!!!!!!deposit!!!!!!!!!!");

    let iter = &mut accounts.iter();

    let payer = next_account_info(iter)?;
    msg!("payer: {}", payer.key);
    let ata_user_wallet = next_account_info(iter)?;
    msg!("ata_user_wallet: {}", ata_user_wallet.key);
    let user_pda = next_account_info(iter)?;
    msg!("user_pda: {}", user_pda.key);
    let program_wallet = next_account_info(iter)?;
    msg!("program_wallet: {}", program_wallet.key);
    let ata_program_wallet = next_account_info(iter)?;
    msg!("ata_program_wallet: {}", ata_program_wallet.key);
    let token_program = next_account_info(iter)?;
    msg!("token_program: {}", token_program.key);
    let spl_associated_token_account_program = next_account_info(iter)?;
    msg!("spl_associated_token_account_program: {}", spl_associated_token_account_program.key);
    let mint = next_account_info(iter)?;
    msg!("mint: {}", mint.key);
    let system_program = next_account_info(iter)?;
    msg!("system_program: {}", system_program.key);

    msg!("!!!!!!!!!!create_program_wallet!!!!!!!!!!");
    create_program_wallet(payer, program_id, program_wallet, system_program)?;

    msg!("!!!!!!!!!!create_ata_program_wallet!!!!!!!!!!");
    create_ata_program_wallet(
        payer,
        program_id,
        token_program,
        spl_associated_token_account_program,
        program_wallet,
        ata_program_wallet,
        mint,
        system_program,
    )?;

    msg!("!!!!!!!!!!create_user_pda!!!!!!!!!!");
    create_user_pda(payer, program_id, user_pda, system_program)?;

    msg!("!!!!!!!!!!transfer!!!!!!!!!!");
    let ix = transfer(
        token_program.key,
        ata_user_wallet.key,
        ata_program_wallet.key,
        payer.key,
        &[],
        amount,
    )?;

    let infos = [
        ata_user_wallet.clone(),
        mint.clone(),
        ata_program_wallet.clone(),
        payer.clone(),
        token_program.clone(),
    ];

    invoke_signed(&ix, &infos, &[])?;

    msg!("!!!!!!!!!!add balance!!!!!!!!!!");
    let mut state = UserData::from_account_mut(user_pda)?; //AccountState::from_account_mut(info)?;
    state.balance = state
        .balance
        .checked_add(amount)
        .ok_or(CalculationOverflow)?;

    Ok(())
}
