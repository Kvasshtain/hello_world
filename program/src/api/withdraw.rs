use {
    crate::{
        accounts::{account_state::AccountState, Data},
        error::Error::CalculationOverflow,
        state::State,
    },
    solana_msg::msg,
    solana_program::{
        account_info::AccountInfo,
        entrypoint_deprecated::ProgramResult,
        program::{invoke, invoke_signed},
    },
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
    spl_associated_token_account_client::instruction::create_associated_token_account_idempotent,
    std::mem,
};

pub fn withdraw<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("withdraw");

    if data.len() < 2 * mem::size_of::<Pubkey>() + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (amount_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());

    let (mint_bytes, to_bytes) = rest.split_at(PUBKEY_BYTES);

    let mint_key = Pubkey::try_from(mint_bytes).unwrap();

    let to_key = Pubkey::try_from(to_bytes).unwrap();

    let state = State::new(program, accounts)?;

    let wallet = state.wallet_info(&mint_key)?;

    let ix = create_associated_token_account_idempotent(
        state.signer().key,
        &wallet.key,
        &state.get(mint_key)?.key,
        &spl_token::ID,
    );

    invoke(&ix, &state.infos(&ix)?)?;

    let user_pda = state.balance_info(state.signer().key, &mint_key)?;

    let (ata_wallet, _bump) = State::spl_ata(&wallet.key, &mint_key);

    let ix = spl_token::instruction::transfer(
        &spl_token::ID,
        &ata_wallet,
        &to_key,
        state.signer().key,
        &[],
        amount,
    )?;

    invoke_signed(&ix, &state.infos(&ix)?, &[])?;

    let mut account_state = AccountState::from_account_mut(user_pda)?;
    account_state.balance = account_state
        .balance
        .checked_sub(amount)
        .ok_or(CalculationOverflow)?;

    Ok(())
}
