use {
    crate::{
        accounts::account_state::AccountState,
        config::WALLET_SEED,
        error::{
            Error,
            Error::{AccountNotFound, InvalidSigner},
        },
    },
    solana_program::{
        account_info::AccountInfo, instruction::Instruction, program::invoke, rent::Rent,
        system_program, sysvar::Sysvar,
    },
    solana_pubkey::Pubkey,
    spl_associated_token_account::tools::account::create_pda_account,
    spl_associated_token_account_client::instruction::create_associated_token_account,
    std::collections::HashMap,
    std::mem,
};

pub type Result<T> = std::result::Result<T, Error>;

pub struct State<'a> {
    program_id: &'a Pubkey,
    all: HashMap<Pubkey, &'a AccountInfo<'a>>,
    signer: &'a AccountInfo<'a>,
}

impl<'a> State<'a> {
    pub fn new(program: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> Result<Self> {
        let all = HashMap::from_iter(accounts.iter().map(|a| *a.key).zip(accounts.iter()));
        let signer = Self::signer_info_static(&all)?;

        Ok(Self {
            program_id: program,
            all,
            signer,
        })
    }

    pub fn signer(&self) -> &Pubkey {
        self.signer.key
    }

    pub fn get(&self, key: Pubkey) -> Result<&'a AccountInfo<'a>> {
        Ok(self.all.get(&key).cloned().ok_or(AccountNotFound(key))?)
    }

    pub fn signer_info(&self) -> Result<&'a AccountInfo<'a>> {
        Self::signer_info_static(&self.all)
    }

    fn signer_info_static(
        map: &HashMap<Pubkey, &'a AccountInfo<'a>>,
    ) -> Result<&'a AccountInfo<'a>> {
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

    fn pda(&self, seeds: &[&[u8]], size: usize, owner: &Pubkey) -> Result<&'a AccountInfo<'a>> {
        let (key, bump) = Pubkey::find_program_address(seeds, self.program_id);

        let pda = self.get(key)?;

        if pda.owner != &system_program::ID {
            return Ok(pda);
        }

        let sys_program = self.get(system_program::ID)?;

        let b = &[bump];

        let mut a = seeds.iter().map(|&x| x).collect::<Vec<_>>();

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

    pub fn wallet_info(&self, mint: Pubkey) -> Result<&'a AccountInfo<'a>> {
        self.pda(
            &[&WALLET_SEED.as_bytes()],
            0,
            self.get(system_program::ID)?.key,
        )
    }

    pub fn aspl_info(&self, wallet: &AccountInfo<'a>, mint_key: Pubkey) -> Result<&'a Pubkey> {
        let mint = self.get(mint_key)?;

        let ata_key = Self::spl_ata(&wallet.key, &mint.key);

        let ata_info = self.get(ata_key.0)?;

        if ata_info.owner == &spl_token::id() {
            return Ok(ata_info.key);
        }

        let ix = create_associated_token_account(
            self.signer.key,
            &wallet.key,
            &mint.key,
            &spl_token::ID,
        );

        invoke(&ix, &self.infos(&ix)?)?;

        Ok(ata_info.key)
    }

    pub fn balance_info(&self, user_key: &Pubkey, mint: &Pubkey) -> Result<&'a AccountInfo<'a>> {
        self.pda(
            &[&user_key.to_bytes(), &mint.to_bytes()],
            mem::size_of::<AccountState>(),
            self.program_id,
        )
    }
}
