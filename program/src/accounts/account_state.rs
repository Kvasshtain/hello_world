use {
    crate::accounts::{cast, cast_mut, Data},
    solana_program::account_info::AccountInfo,
    std::cell::{Ref, RefMut},
    std::mem::size_of,
};

#[repr(C, packed)]
pub struct AccountState {
    pub(crate) balance: u64,
}

impl Data for AccountState {
    type Item<'a> = Ref<'a, Self>;
    type ItemMut<'a> = RefMut<'a, Self>;

    fn from_account<'a>(info: &'a AccountInfo) -> crate::api::deposit::Result<Self::Item<'a>> {
        cast(info, 0, size_of::<Self>())
    }

    fn from_account_mut<'a>(
        info: &'a AccountInfo,
    ) -> crate::api::deposit::Result<Self::ItemMut<'a>> {
        cast_mut(info, 0, size_of::<Self>())
    }
}
