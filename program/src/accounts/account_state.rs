use {
    crate::{
        accounts::{account_type::AccountType, cast, cast_mut, Data},
        error::Error,
    },
    solana_program::account_info::AccountInfo,
    std::{
        cell::{Ref, RefMut},
        fmt::Debug,
        mem::size_of,
    },
};
use crate::accounts::account_lock::AccountLock;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AccountState {
    pub balance: u64,
}

impl AccountState {
    pub fn init(info: &AccountInfo) -> Result<(), Error> {
        AccountLock::init(info, AccountType::Balance)?;

        let len = AccountState::offset(info) + AccountState::size(info);
        assert_eq!(len, info.data_len());

        let mut state = AccountState::from_account_mut(info)?;

        *state = AccountState {
            balance: 0,
        };

        Ok(())
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.balance.to_le_bytes().to_vec()
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
    fn offset(info: &AccountInfo) -> usize {
        AccountLock::offset(info) + AccountLock::size(info)
    }
}
