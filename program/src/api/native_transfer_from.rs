use {
    crate::{error::Error::AlreadyOwned, state::State},
    solana_msg::msg,
    solana_program::{
        account_info::AccountInfo, entrypoint_deprecated::ProgramResult, program::invoke_signed,
        system_instruction, system_program,
    },
    solana_program_error::ProgramError,
    solana_pubkey::{Pubkey, PUBKEY_BYTES},
    std::mem,
};

pub fn native_transfer_from<'a>(
    program: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    data: &[u8],
) -> ProgramResult {
    msg!("transfer_from");

    if data.len() <= PUBKEY_BYTES + mem::size_of::<u64>() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (to_bytes, rest) = data.split_at(PUBKEY_BYTES);
    let to_pubkey = Pubkey::new_from_array(to_bytes.try_into().unwrap());
    let (amount_bytes, seed_bytes) = rest.split_at(mem::size_of::<u64>());
    let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
    let seed: &[u8] = seed_bytes.try_into().unwrap();

    let state = State::new(program, accounts)?;

    let to = state.get(to_pubkey)?;

    let (from_key, bump) = Pubkey::find_program_address(&[seed], program);
    let from = state.get(from_key)?;

    if from.owner == program {
        msg!("<<<borrow>>>");

        if from_key != *from.key {
            return Err(ProgramError::from(AlreadyOwned));
        }

        **from.try_borrow_mut_lamports()? -= amount;
        **to.try_borrow_mut_lamports()? += amount;

        return Ok(());
    }

    if from.owner == &system_program::ID {
        msg!("<<<transfer>>>");

        let ix = system_instruction::transfer(from.key, to.key, amount);

        invoke_signed(&ix, &state.infos(&ix)?, &[&[seed, &[bump]]])?;

        return Ok(());
    }

    Err(ProgramError::from(AlreadyOwned))
}
