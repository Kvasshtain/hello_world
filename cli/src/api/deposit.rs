use {
    crate::{
        deposit_transactions::build_deposit_tx,
    },
    anyhow::Result,
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_sdk::{
        pubkey::Pubkey,
        signature::Signer,
        signature::{Keypair, Signature},
    },
    spl_associated_token_account_client::{
        address::get_associated_token_address_and_bump_seed_internal,
    },
    std::str::FromStr,
};

pub async fn deposit(program_id: Pubkey,
                     keypair: Keypair,
                     client: &RpcClient,
                     amount: u64,
                     mint: String,
) -> Result<Signature> {
    let mut data = vec![6];
    data.extend(amount.to_le_bytes());
    let (usr_pda_key, _bump) =
        Pubkey::find_program_address(&[&keypair.pubkey().to_bytes()], &program_id);
    let mint = Pubkey::from_str(mint.as_str())?;
    data.extend(mint.to_bytes());
    let (program_wallet_key, _bump) =
        Pubkey::find_program_address(&[WALLET_SEED], &program_id);
    let (ata_user_wallet, _bump) =
        get_associated_token_address_and_bump_seed_internal(
            &keypair.pubkey(),
            &mint,
            &spl_associated_token_account::ID,
            &spl_token::ID,
        );
    let (ata_program_wallet_key, _bump) =
        get_associated_token_address_and_bump_seed_internal(
            &program_wallet_key,
            &mint,
            &spl_associated_token_account::ID,
            &spl_token::ID,
        );
    build_deposit_tx(
        program_id,
        data,
        &client,
        keypair,
        ata_user_wallet,
        usr_pda_key,
        program_wallet_key,
        ata_program_wallet_key,
        spl_token::ID,
        spl_associated_token_account::ID,
        mint,
    ).await
}