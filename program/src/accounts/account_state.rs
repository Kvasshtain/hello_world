use {
    crate::{
        accounts::{cast, cast_mut, Data},
        config::LOCK_DURATION,
        error::Error,
    },
    solana_program::{account_info::AccountInfo, clock::Clock, pubkey::Pubkey, sysvar::Sysvar},
    std::{
        cell::{Ref, RefMut},
        fmt::Debug,
        mem::size_of,
    },
};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AccountState {
    pub balance: u64,
    pub old_balance: u64,
    pub lock: bool,
    // pub holder: [u8; 32],
    timestamp: i64,
}

impl AccountState {
    pub fn init(info: &AccountInfo) -> Result<(), Error> {
        let mut state = AccountState::from_account_mut(info)?;

        *state = AccountState {
            balance: 0,
            old_balance: 0,
            lock: false,
            // holder: [0; 32],
            timestamp: 0,
        };

        Ok(())
    }

    fn is_expired(&self) -> Result<bool, Error> {
        let expired = Clock::get()?
            .unix_timestamp
            .checked_sub(self.timestamp)
            .ok_or(Error::CalculationUnderflow)?
            >= LOCK_DURATION;

        Ok(expired)
    }

    pub fn get_lock(&mut self) -> Result<bool, Error> {
        if !self.lock {
            return Ok(false);
        }

        if self.is_expired()? {
            self.balance = self.old_balance;
            self.lock = false;
            
            return Ok(false);
        }

        Ok(true)
    }

    pub fn lock(&mut self) -> Result<(), Error> {
        self.lock = true;
        self.timestamp = Clock::get()?.unix_timestamp;
        self.old_balance = self.balance;
        Ok(())
    }

    pub fn unlock(&mut self) {
        self.lock = false;
    }

    pub fn unlock_err(&mut self) {
        self.balance = self.old_balance;
        self.lock = false;
    }

    pub fn update_timestamp(&mut self) -> Result<(), Error> {
        self.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn is_new_one(&self) -> bool {
        !self.lock && self.timestamp == 0
    }
}

impl Data for AccountState {
    type Item<'a> = Ref<'a, Self>;
    type ItemMut<'a> = RefMut<'a, Self>;

    fn from_account<'a>(info: &'a AccountInfo) -> Result<Self::Item<'a>, Error> {
        cast(info, Self::offset(info), Self::size(info))
    }

    fn from_account_mut<'a>(info: &'a AccountInfo) -> Result<Self::ItemMut<'a>, Error> {
        cast_mut(info, Self::offset(info), Self::size(info))
    }

    fn size(_info: &AccountInfo) -> usize {
        size_of::<Self>()
    }

    fn offset(_info: &AccountInfo) -> usize {
        0
    }
}
