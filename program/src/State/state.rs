use solana_msg::msg;
use spl_associated_token_account_client::address::get_associated_token_address_and_bump_seed_internal;
use {
    crate::{
        config::{DATA_SIZE, WALLET_SEED},
        error::{
            Error,
            Error::{AccountNotFound, InvalidSigner},
        },
    },
    solana_account_info::AccountInfo,
    solana_program::{
        program::{invoke, invoke_signed},
        rent::Rent,
        system_program,
        sysvar::Sysvar,
    },
    solana_program_entrypoint::ProgramResult,
    solana_pubkey::Pubkey,
    spl_associated_token_account::tools::account::create_pda_account,
    spl_associated_token_account_client::{
        address::get_associated_token_address_with_program_id,
        instruction::create_associated_token_account,
    },
    spl_token::instruction::transfer,
    std::collections::HashMap,
};


pub type Result<T> = std::result::Result<T, Error>;

pub struct State<'a> {
    program: &'a Pubkey,
    all: HashMap<Pubkey, &'a AccountInfo<'a>>,
    signer: &'a AccountInfo<'a>,
}

impl<'a> State<'a> {
    pub fn new(
        program: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
    ) -> Result<Self> {
        let all = HashMap::from_iter(accounts.iter().map(|a| *a.key).zip(accounts.iter()));
        let signer = Self::get_signer(&all)?;

        Ok(Self { program, all, signer, })
    }

    fn find_account(
        &self,
        key: Pubkey,
    ) -> Result<&'a AccountInfo<'a>> {
        let info: &AccountInfo = self.all.get(&key).cloned().ok_or(AccountNotFound(key))?;
        Ok(info)
    }
    
    fn get_signer(map: &HashMap<Pubkey, &'a AccountInfo<'a>>) -> Result<&'a AccountInfo<'a>> {
        let mut signer = None;

        for &info in map.values() {
            if info.is_signer && info.is_writable {
                if signer.is_some() {
                    return Err(InvalidSigner);
                }
                signer = Some(info)
            }
        }

        signer.ok_or(InvalidSigner)
    }

    pub fn get_wallet(
        &self,
    ) -> Result<&'a AccountInfo<'a>> {
        let (wallet_key, bump) = Pubkey::find_program_address(&[WALLET_SEED], self.program);

        let wallet = self.find_account(wallet_key)?;

        if wallet.owner != &system_program::ID  {
            return Ok(wallet)
        }

        let sys_program = self.find_account(system_program::ID)?;

        create_pda_account(
            self.signer,
            &Rent::get()?,
            DATA_SIZE,
            self.program,
            sys_program,
            wallet,
            &[WALLET_SEED, &[bump]],
        )?;

        Ok(wallet)
    }

    pub fn get_ata_wallet(
        &self,
        wallet: &AccountInfo<'a>,
        mint_key: Pubkey,
    ) -> Result<&'a AccountInfo<'a>> {
        let mint = self.find_account(mint_key)?;

        let spl_ata_program = self.find_account(spl_associated_token_account::ID)?;

        let ata_wallet_key =
            get_associated_token_address_with_program_id(&wallet.key, &mint.key, &spl_token::ID);

        let ata_wallet = self.find_account(ata_wallet_key)?;

        if ata_wallet.owner != &system_program::ID  {
            return Ok(ata_wallet)
        }

        let sys_program = self.find_account(system_program::ID)?;

        let ix =
            create_associated_token_account(self.signer.key, &wallet.key, &mint.key, &spl_token::ID);

        let infos = [
            self.signer.clone(),
            ata_wallet.clone(),
            wallet.clone(),
            mint.clone(),
            sys_program.clone(),
            spl_ata_program.clone(),
        ];

        invoke(&ix, &infos)?;

        Ok(ata_wallet)
    }

    pub fn get_user_pda(
        &self,
    ) -> Result<&'a AccountInfo<'a>> {
        let (user_pda_key, usr_pda_bump) = Pubkey::find_program_address(&[&self.signer.key.to_bytes()], self.program);

        let user_pda = self.find_account(user_pda_key)?;

        if user_pda.owner != &system_program::ID  {
            return Ok(user_pda)
        }

        let sys_program = self.find_account(system_program::ID)?;

        create_pda_account(
            self.signer,
            &Rent::get()?,
            DATA_SIZE,
            self.program,
            sys_program,
            user_pda,
            &[&self.signer.key.to_bytes(), &[usr_pda_bump]],
        )?;

        Ok(user_pda)
    }

    pub fn transfer(
        &self,
        ata_wallet: &AccountInfo<'a>,
        mint_key: Pubkey,
        amount: u64,
    ) -> ProgramResult {
        let mint = self.find_account(mint_key)?;

        let token_program = self.find_account(spl_token::ID)?;

        let (ata_user_wallet_key, _bump) =
            get_associated_token_address_and_bump_seed_internal(
                &self.signer.key,
                &mint.key,
                &spl_associated_token_account::ID,
                &spl_token::ID,
            );

        let ata_user_wallet = self.find_account(ata_user_wallet_key)?;

        let ix = transfer(
            token_program.key,
            ata_user_wallet.key,
            ata_wallet.key,
            self.signer.key,
            &[],
            amount,
        )?;

        let infos = [
            ata_user_wallet.clone(),
            mint.clone(),
            ata_wallet.clone(),
            self.signer.clone(),
            token_program.clone(),
        ];

        invoke_signed(&ix, &infos, &[])?;

        Ok(())
    }
}