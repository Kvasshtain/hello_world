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
use crate::State::State;

pub type Result<T> = std::result::Result<T, Error>;

fn find_account<'a>(
    all: &HashMap<Pubkey, &'a AccountInfo<'a>>,
    key: Pubkey,
) -> Result<&'a AccountInfo<'a>> {
    let info: &AccountInfo = all.get(&key).cloned().ok_or(AccountNotFound(key))?;
    Ok(info)
}

pub fn deposit<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("deposit");

    if data.len() <= PUBKEY_BYTES + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (amount_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
    let mint_key = Pubkey::new_from_array(rest.try_into().unwrap());

    let state = State::new(program, accounts)?;

    let wallet = state.get_wallet()?;

    let ata_wallet = state.get_ata_wallet(wallet, mint_key)?;

    let user_pda = state.get_user_pda()?;

    state.transfer(ata_wallet, mint_key, amount)?;

    assert!(user_pda.is_writable);

    let mut account_state = AccountState::from_account_mut(user_pda)?;
    account_state.balance = account_state
        .balance
        .checked_add(amount)
        .ok_or(CalculationOverflow)?;

    Ok(())
}
