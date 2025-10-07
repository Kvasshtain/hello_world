use {
    crate::context::Context,
    anyhow::Result,
    hello_world::config::WALLET_SEED,
    hello_world::Instruction,
    solana_sdk::{pubkey::Pubkey, signature::Signature, signature::Signer},
    spl_associated_token_account_client::address::get_associated_token_address_and_bump_seed_internal,
};

pub async fn deposit<'a>(context: Context<'a>, amount: u64, mint: Pubkey) -> Result<Signature> {
    let mut data = vec![Instruction::Deposit as u8];

    data.extend(amount.to_le_bytes());

    data.extend(mint.to_bytes());

    let (usr_pda_key, _bump) =
        Pubkey::find_program_address(&[&context.keypair.pubkey().to_bytes()], &context.program_id);

    let (program_wallet_key, _bump) =
        Pubkey::find_program_address(&[WALLET_SEED], &context.program_id);

    let (ata_user_wallet, _bump) = get_associated_token_address_and_bump_seed_internal(
        &context.keypair.pubkey(),
        &mint,
        &spl_associated_token_account::ID,
        &spl_token::ID,
    );

    let (ata_program_wallet_key, _bump) = get_associated_token_address_and_bump_seed_internal(
        &program_wallet_key,
        &mint,
        &spl_associated_token_account::ID,
        &spl_token::ID,
    );

    let ix = context.compose_ix(
        &data.as_slice(),
        &[
            &ata_user_wallet,
            &usr_pda_key,
            &program_wallet_key,
            &ata_program_wallet_key,
            &spl_token::ID,
            &spl_associated_token_account::ID,
            &mint,
        ],
    );

    let tx = context.compose_tx(&[ix]).await?;

    Ok(context.client.send_and_confirm_transaction(&tx).await?)
}
