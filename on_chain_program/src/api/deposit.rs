use {
    crate::{
        accounts::{account_state::AccountState, Data},
        config::{DATA_SIZE, WALLET_SEED},
        error::{
            Error,
            Error::{AccountNotFound, CalculationOverflow},
        },
    },
    solana_account_info::AccountInfo,
    solana_msg::msg,
    solana_program::{
        program::{invoke, invoke_signed},
        rent::Rent,
        system_program,
        sysvar::Sysvar,
    },
    solana_program_entrypoint::ProgramResult,
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_pubkey::PUBKEY_BYTES,
    spl_associated_token_account::tools::account::create_pda_account,
    spl_associated_token_account_client::{
        address::get_associated_token_address_with_program_id,
        instruction::create_associated_token_account,
    },
    spl_token::instruction::transfer,
    std::collections::HashMap,
    std::mem,
};

pub type Result<T> = std::result::Result<T, Error>;

fn find_account<'a>(
    all: &HashMap<Pubkey, &'a AccountInfo<'a>>,
    key: Pubkey,
) -> Result<&'a AccountInfo<'a>> {
    let info: &AccountInfo = all.get(&key).cloned().ok_or(AccountNotFound(key))?;
    Ok(info)
}

pub fn deposit<'a>(
    program: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("deposit");

    if data.len() <= 3 * PUBKEY_BYTES + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (amount_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
    let (payer_key_bytes, rest) = rest.split_at(PUBKEY_BYTES);
    let payer_key = Pubkey::new_from_array(payer_key_bytes.try_into().unwrap());
    let (ata_user_wallet_key_bytes, mint_key_bytes) = rest.split_at(PUBKEY_BYTES);
    let ata_user_wallet_key = Pubkey::new_from_array(ata_user_wallet_key_bytes.try_into().unwrap());
    let mint_key = Pubkey::new_from_array(mint_key_bytes.try_into().unwrap());

    let all: HashMap<Pubkey, &AccountInfo> =
        HashMap::from_iter(accounts.iter().map(|a| *a.key).zip(accounts.iter()));

    let payer = all
        .get(&payer_key)
        .cloned()
        .ok_or(ProgramError::InvalidInstructionData)?;

    assert!(payer.is_signer);

    let ata_user_wallet = find_account(&all, ata_user_wallet_key)?;

    let (usr_pda_key, usr_pda_bump) =
        Pubkey::find_program_address(&[&payer.key.to_bytes()], program);

    let user_pda = find_account(&all, usr_pda_key)?;

    let (wallet_key, bump) = Pubkey::find_program_address(&[WALLET_SEED], program);

    let wallet = find_account(&all, wallet_key)?;

    let token_program = find_account(&all, spl_token::ID)?;

    let spl_ata_program = find_account(&all, spl_associated_token_account::ID)?;

    let mint = find_account(&all, mint_key)?;

    let sys_program = find_account(&all, system_program::ID)?;

    let ata_wallet_key =
        get_associated_token_address_with_program_id(&wallet.key, &mint.key, &spl_token::ID);

    let ata_wallet = find_account(&all, ata_wallet_key)?;

    if wallet.lamports() == 0 || wallet.owner == &system_program::ID {
        create_pda_account(
            payer,
            &Rent::get()?,
            DATA_SIZE,
            program,
            sys_program,
            wallet,
            &[WALLET_SEED, &[bump]],
        )?;
    }

    assert_eq!(wallet.owner, program);

    if ata_wallet.lamports() == 0 {
        if ata_wallet.owner != &system_program::ID {
            return Err(ProgramError::InvalidInstructionData);
        }

        let ix =
            create_associated_token_account(&payer.key, &wallet.key, &mint.key, &spl_token::ID);

        let infos = [
            payer.clone(),
            ata_wallet.clone(),
            wallet.clone(),
            mint.clone(),
            sys_program.clone(),
            spl_ata_program.clone(),
        ];

        invoke(&ix, &infos)?;
    }

    assert_eq!(ata_wallet.owner, spl_ata_program.key);

    if user_pda.lamports() == 0 || user_pda.owner == &system_program::ID {
        create_pda_account(
            payer,
            &Rent::get()?,
            DATA_SIZE,
            program,
            sys_program,
            user_pda,
            &[&payer.key.to_bytes(), &[usr_pda_bump]],
        )?;
    }

    let ix = transfer(
        token_program.key,
        ata_user_wallet.key,
        ata_wallet.key,
        payer.key,
        &[],
        amount,
    )?;

    let infos = [
        ata_user_wallet.clone(),
        mint.clone(),
        ata_wallet.clone(),
        payer.clone(),
        token_program.clone(),
    ];

    invoke_signed(&ix, &infos, &[])?;

    assert!(user_pda.is_writable);

    let mut state = AccountState::from_account_mut(user_pda)?;
    state.balance = state
        .balance
        .checked_add(amount)
        .ok_or(CalculationOverflow)?;

    Ok(())
}
