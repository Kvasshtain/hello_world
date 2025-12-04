use {
    crate::{
        state::{State, Result},
    },
    solana_program::{
        account_info::AccountInfo,
        program::invoke,
    },
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    spl_associated_token_account_client::instruction::create_associated_token_account_idempotent,
    std::mem,
};

pub struct DepositWithdrawData<'a> {
    pub state: State<'a>,
    pub ata_wallet: Pubkey,
    pub ata_user_wallet_key: Pubkey,
    pub user_pda: &'a AccountInfo<'a>,
    pub amount: u64,
}

pub fn deposit_withdraw_data<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> Result<DepositWithdrawData<'a>> {

    if data.len() < mem::size_of::<Pubkey>() + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData.into());
    }

    let (amount_bytes, rest) = data.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
    let mint_key = Pubkey::try_from(rest).unwrap();

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

    let (ata_user_wallet_key, _bump) = State::spl_ata(state.signer().key, &mint_key);

    Ok(DepositWithdrawData {
        state,
        ata_wallet,
        ata_user_wallet_key,
        user_pda,
        amount,
    })
}