use {
    super::{cast, cast_mut, Data, Result},
    crate::{
        accounts::{
            account_type::AccountType::{self, Balance, Storage},
            ver::Ver,
        },
        config::LOCK_DURATION,
        error::Error,
    },
    solana_program::{account_info::AccountInfo, clock::Clock, pubkey::Pubkey, sysvar::Sysvar},
    std::{
        cell::{Ref, RefMut},
        fmt::{self, Debug, Formatter},
        mem::size_of,
    },
};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct HolderLock {
    // pub lock: bool,
    timestamp: i64,
}

impl HolderLock {
    pub fn init(info: &AccountInfo, typ: AccountType) -> std::result::Result<(), Error> {
        Ver::init(info, typ)?;

        let mut lock = HolderLock::from_account_mut(info)?;
        *lock = HolderLock {
            // lock: false,
            timestamp: 0,
        };

        Ok(())
    }
    
    // pub fn is_managed(info: &AccountInfo, program_id: &Pubkey) -> Result<bool> {
    //     if AccountType::check_owner(info, program_id).is_ok() {
    //         let typ = AccountType::from_account(info)?;
    //         return Ok(*typ == Balance || *typ == Storage);
    //     }
    // 
    //     Ok(false)
    // }
    
    fn is_expired(&self) -> Result<bool> {
        let expired = Clock::get()?
            .unix_timestamp
            .checked_sub(self.timestamp)
            .ok_or(Error::CalculationUnderflow)?
            >= LOCK_DURATION;

        Ok(expired)
    }
    pub fn get(&self) -> Result<bool> {
        // let mut lock = false;

        if self.is_expired()? {
            return Ok(false);
        }

        Ok(true)
    }
    
    pub fn lock(&mut self) -> std::result::Result<(), Error> {
        self.update()?;
        Ok(())
    }
    
    pub fn unlock(&mut self) {
        self.timestamp = 0
    }
    pub fn update(&mut self) -> std::result::Result<(), Error> {
        self.timestamp = Clock::get()?.unix_timestamp;
        Ok(())
    }
    pub fn is_new_one(&self) -> bool {
        self.timestamp == 0
    }
}

impl Data for HolderLock {
    type Item<'a> = Ref<'a, Self>;
    type ItemMut<'a> = RefMut<'a, Self>;

    fn from_account<'a>(info: &'a AccountInfo) -> Result<Self::Item<'a>> {
        cast(info, Self::offset(info), Self::size(info))
    }
    fn from_account_mut<'a>(info: &'a AccountInfo) -> Result<Self::ItemMut<'a>> {
        cast_mut(info, Self::offset(info), Self::size(info))
    }
    fn size(_info: &AccountInfo) -> usize {
        size_of::<Self>()
    }
    fn offset(info: &AccountInfo) -> usize {
        Ver::offset(info) + Ver::size(info)
    }
}
