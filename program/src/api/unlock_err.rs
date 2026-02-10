use {
    crate::{
        accounts::{account_state::AccountState, Data},
        state::State,
    },
    solana_msg::msg,
    solana_program::{account_info::AccountInfo, entrypoint_deprecated::ProgramResult},
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
};

// pub fn unlock_err<'a>(
//     program: &'a Pubkey,
//     accounts: &'a [AccountInfo<'a>],
//     data: &[u8],
// ) -> ProgramResult {
//     msg!("lock");
//
//     if data.len() < 2 * PUBKEY_BYTES {
//         msg!("Error1");
//         return Err(ProgramError::InvalidInstructionData);
//     }
//
//     let (mint_bytes, pubkey_bytes) = data.split_at(PUBKEY_BYTES);
//
//     let mint_key = Pubkey::try_from(mint_bytes).unwrap();
//
//     let pubkey = Pubkey::try_from(pubkey_bytes).unwrap();
//
//     let state = State::new(program, accounts)?;
//
//     let balance_pda = state.balance_info(&pubkey, &mint_key)?;
//
//     let mut account_state = AccountState::from_account_mut(balance_pda)?;
//     // account_state.unlock_err();
//
//     Ok(())
// }
