use {
    crate::{
        accounts::{account_state::AccountState, Data},
        error::{Error, Error::CalculationOverflow},
        state::State,
    },
    solana_msg::msg,
    solana_program::{
        account_info::AccountInfo, entrypoint_deprecated::ProgramResult, program::invoke_signed,
    },
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    std::mem,
};

pub type Result<T> = std::result::Result<T, Error>;

fn transfer(state: &State, ata_wallet: &Pubkey, mint_key: Pubkey, amount: u64) -> ProgramResult {
    let token_program = state.get(spl_token::ID)?;

    let (ata_user_wallet_key, _bump) = State::spl_ata(state.signer(), &mint_key);

    let ata_user_wallet = state.get(ata_user_wallet_key)?;

    let ix = spl_token::instruction::transfer(
        token_program.key,
        ata_user_wallet.key,
        ata_wallet,
        state.signer(),
        &[],
        amount,
    )?;

    invoke_signed(&ix, &state.infos(&ix)?, &[])?;

    Ok(())
}

pub fn deposit<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("deposit");

    if data.len() < mem::size_of::<Pubkey>() + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (amount_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
    let mint_key = Pubkey::new_from_array(rest.try_into().unwrap());

    let state = State::new(program, accounts)?;

    let wallet = state.wallet_info()?;

    let ata_wallet = state.aspl_info(wallet, mint_key)?;

    let user_pda = state.balance_info()?;

    transfer(&state, ata_wallet, mint_key, amount)?;

    let mut account_state = AccountState::from_account_mut(user_pda)?;
    account_state.balance = account_state
        .balance
        .checked_add(amount)
        .ok_or(CalculationOverflow)?;

    Ok(())
}
