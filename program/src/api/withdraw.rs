use {
    crate::{
        accounts::{account_state::AccountState, Data},
        error::Error::CalculationOverflow,
        api::deposit_withdraw_data::deposit_withdraw_data,
    },
    solana_msg::msg,
    solana_program::{
        account_info::AccountInfo,
        entrypoint_deprecated::ProgramResult,
        program::invoke_signed,
    },
    solana_pubkey::Pubkey,
};

pub fn withdraw<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("withdraw");

    let data = deposit_withdraw_data(program, accounts, data)?;

    let ix = spl_token::instruction::transfer(
        &spl_token::ID,
        &data.ata_wallet,
        &data.ata_user_wallet_key,
        data.state.signer().key,
        &[],
        data.amount,
    )?;

    invoke_signed(&ix, &data.state.infos(&ix)?, &[])?;

    let mut account_state = AccountState::from_account_mut(data.user_pda)?;
    account_state.balance = account_state
        .balance
        .checked_sub(data.amount)
        .ok_or(CalculationOverflow)?;

    Ok(())
}
