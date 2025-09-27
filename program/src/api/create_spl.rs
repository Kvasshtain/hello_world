use {solana_account_info::AccountInfo, solana_program_entrypoint::ProgramResult};

pub fn create_spl(accounts: &[AccountInfo]) -> ProgramResult {
    // let iter = &mut accounts.iter();
    //
    // let mint_authority = next_account_info(iter)?;
    // let mint = next_account_info(iter)?;
    // let token_account = next_account_info(iter)?;
    // let rent = next_account_info(iter)?;
    // let token_program = next_account_info(iter)?;
    // let associated_token_program = next_account_info(iter)?;
    // let _system = next_account_info(iter)?;
    //
    // msg!("create mint account");
    // msg!("mint.key: {}", mint.key);
    //
    // let ix_create_mint = &instruction::create_account(
    //     mint_authority.key,
    //     mint.key,
    //     Rent::get()?.minimum_balance(MINT_SIZE),
    //     MINT_SIZE as u64,
    //     &token_program.key,
    // );
    //
    // let infos_create_mint = vec![mint_authority.clone(), mint.clone(), token_program.clone()];
    //
    // invoke(&ix_create_mint, &infos_create_mint)?;
    //
    // msg!("initialize mint account");
    // msg!("mint.key: {}", mint.key);
    //
    // let ix_init_mint = &token_instruction::initialize_mint(
    //     token_program.key,
    //     mint.key,
    //     mint_authority.key,
    //     Some(mint_authority.key),
    //     0,
    // )?;
    //
    // let infos_init_mint = vec![
    //     mint.clone(),
    //     mint_authority.clone(),
    //     token_program.clone(),
    //     rent.clone(),
    // ];
    //
    // invoke(ix_init_mint, &infos_init_mint)?;
    //
    // msg!("create token account");
    // msg!("token_account.key: {}", token_account.key);
    //
    // invoke(
    //     &create_associated_token_account(
    //         &mint_authority.key,
    //         &mint_authority.key,
    //         &mint.key,
    //         token_program.key,
    //     ),
    //     &[
    //         mint.clone(),
    //         token_account.clone(),
    //         mint_authority.clone(),
    //         token_program.clone(),
    //         associated_token_program.clone(),
    //     ],
    // )?;
    //
    // msg!("Mint token to token account");
    // msg!("mint.key: {}", mint.key);
    // msg!("token_account.key: {}", token_account.key);
    //
    // invoke(
    //     &token_instruction::mint_to(
    //         token_program.key,
    //         &mint.key,
    //         &token_account.key,
    //         &mint_authority.key,
    //         &[&mint_authority.key],
    //         1,
    //     )?,
    //     &[
    //         mint.clone(),
    //         mint_authority.clone(),
    //         token_account.clone(),
    //         token_program.clone(),
    //         rent.clone(),
    //     ],
    // )?;

    Ok(())
}
