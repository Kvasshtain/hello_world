use {
    crate::{
        accounts::account_state::AccountState,
        base::Base,
        config::{BALANCE_ACCOUNT, WALLET_SEED},
        error::{
            Error,
            Error::{AccountNotFound, InvalidSigner},
        },
        seed::Seed,
    },
    solana_program::{
        account_info::AccountInfo, instruction::Instruction, rent::Rent, system_program,
        sysvar::Sysvar,
    },
    solana_pubkey::Pubkey,
    spl_associated_token_account::tools::account::create_pda_account,
    std::{collections::HashMap, mem, ops::Deref},
};

pub type Result<T> = std::result::Result<T, Error>;

pub struct State<'a> {
    pub base: Base<'a>,
    all: HashMap<Pubkey, &'a AccountInfo<'a>>,
    pub signer: &'a AccountInfo<'a>,
}

impl<'a> Deref for State<'a> {
    type Target = Base<'a>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'a> State<'a> {
    pub fn new(program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> Result<Self> {
        let all = HashMap::from_iter(accounts.iter().map(|a| *a.key).zip(accounts.iter()));
        let signer = Self::signer_static(&all)?;

        Ok(Self {
            base: Base::new(program_id),
            all,
            signer,
        })
    }

    fn signer_static(map: &HashMap<Pubkey, &'a AccountInfo<'a>>) -> Result<&'a AccountInfo<'a>> {
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

    pub fn get(&self, key: Pubkey) -> Result<&'a AccountInfo<'a>> {
        self.all.get(&key).cloned().ok_or(AccountNotFound(key))
    }

    pub fn signer(&self) -> &AccountInfo<'a> {
        self.signer
    }

    pub fn infos(&self, ix: &Instruction) -> Result<Vec<AccountInfo<'a>>> {
        let f_info = |key: Pubkey| {
            self.all
                .get(&key)
                .map(|&x| x.clone())
                .ok_or(AccountNotFound(key))
        };

        ix.accounts
            .iter()
            .map(|a| f_info(a.pubkey))
            .collect::<Result<Vec<AccountInfo>>>()
    }

    fn pda(
        &self,
        pubkey_bump_seeds: (Pubkey, u8, Seed),
        size: usize,
        owner: &Pubkey,
    ) -> Result<&'a AccountInfo<'a>> {
        let pda = self.get(pubkey_bump_seeds.0)?;

        if pda.owner != &system_program::ID {
            return Ok(pda);
        }

        let sys_program = self.get(system_program::ID)?;

        let b = &[pubkey_bump_seeds.1];

        let mut a = pubkey_bump_seeds
            .2
            .cast()
            .as_slice()
            .iter()
            .map(|&x| x)
            .collect::<Vec<_>>();

        a.push(b);

        create_pda_account(
            self.signer,
            &Rent::get()?,
            size,
            owner,
            sys_program,
            pda,
            &a,
        )?;

        Ok(pda)
    }

    pub fn spl_ata(owner: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                &owner.to_bytes(),
                &spl_token::ID.to_bytes(),
                &mint.to_bytes(),
            ],
            &spl_associated_token_account::ID,
        )
    }

    pub fn wallet_pubkey_bump(program_id: &'a Pubkey, mint: &'a Pubkey) -> (Pubkey, u8, Seed) {
        let seeds = Seed {
            items: vec![WALLET_SEED.to_vec(), mint.as_ref().to_vec()],
        };

        let result = Pubkey::find_program_address(&seeds.cast().as_slice(), program_id);

        (result.0, result.1, seeds)
    }

    pub fn wallet_info(&self, mint: &Pubkey) -> Result<&'a AccountInfo<'a>> {
        let pubkey_bump_seeds = State::wallet_pubkey_bump(self.program_id, mint);

        self.pda(pubkey_bump_seeds, 0, &system_program::ID)
    }

    pub fn balance_pubkey_bump(
        program_id: &'a Pubkey,
        user_key: &'a Pubkey,
        mint: &'a Pubkey,
    ) -> (Pubkey, u8, Seed) {
        let seeds = Seed {
            items: vec![
                BALANCE_ACCOUNT.to_vec(),
                user_key.as_ref().to_vec(),
                mint.as_ref().to_vec(),
            ],
        };

        let result = Pubkey::find_program_address(&seeds.cast().as_slice(), program_id);

        (result.0, result.1, seeds)
    }

    pub fn balance_info(&self, user_key: &Pubkey, mint: &Pubkey) -> Result<&'a AccountInfo<'a>> {
        let pubkey_bump_seeds = State::balance_pubkey_bump(self.program_id, user_key, mint);

        self.pda(
            pubkey_bump_seeds,
            mem::size_of::<AccountState>(),
            self.program_id,
        )
    }
}
