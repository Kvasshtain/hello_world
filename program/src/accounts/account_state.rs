use {
    crate::{
        accounts::{cast, cast_mut, Data},
        error::Error,
    },
    solana_program::account_info::AccountInfo,
    std::cell::{Ref, RefMut},
    std::mem::size_of,
};
pub(crate) use crate::accounts::lock::Lock;

#[repr(C, packed)]
pub struct AccountState {
    pub balance: u64,
}

impl Data for AccountState {
    type Item<'a> = Ref<'a, Self>;
    type ItemMut<'a> = RefMut<'a, Self>;

    fn from_account<'a>(info: &'a AccountInfo) -> Result<Self::Item<'a>, Error> {
        cast(info, 0, size_of::<Self>())
    }

    fn from_account_mut<'a>(info: &'a AccountInfo) -> Result<Self::ItemMut<'a>, Error> {
        cast_mut(info, 0, size_of::<Self>())
    }

    fn size(_info: &AccountInfo) -> usize {
        size_of::<Self>()
    }
    fn offset(info: &AccountInfo) -> usize {
        Lock::offset(info) + Lock::size(info)
    }
}
