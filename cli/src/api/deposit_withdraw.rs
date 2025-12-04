use {
    crate::context::Context,
    anyhow::Result,
    hello_world::{Instruction, State},
    solana_sdk::{pubkey::Pubkey, signature::Signature, signature::Signer},
};

pub async fn deposit_withdraw<'a>(context: Context<'a>, amount: u64, mint: Pubkey, instruction: Instruction) -> Result<Signature> {
    let mut data = vec![instruction as u8];

    data.extend(amount.to_le_bytes());

    data.extend(mint.to_bytes());

    let (balance_key, _bump, _seed) =
        State::balance_pubkey_bump(&context.program_id, &context.keypair.pubkey(), &mint);

    let (program_wallet_key, _bump, _seed) = State::wallet_pubkey_bump(&context.program_id, &mint);

    let (ata_user_wallet, _bump) = State::spl_ata(&context.keypair.pubkey(), &mint);

    let (ata_program_wallet_key, _bump) = State::spl_ata(&program_wallet_key, &mint);

    let ix = context.compose_ix(
        &data.as_slice(),
        &[
            &ata_user_wallet,
            &balance_key,
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
