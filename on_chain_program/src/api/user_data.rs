use {
    crate::api::{cast, cast_mut, Data},
    solana_account_info::AccountInfo,
    std::cell::{Ref, RefMut},
    std::mem::size_of,
};

#[repr(C, packed)]
pub struct UserData {
    pub(crate) balance: u64,
}

impl Data for UserData {
    type Item<'a> = Ref<'a, Self>;
    type ItemMut<'a> = RefMut<'a, Self>;

    fn from_account<'a>(info: &'a AccountInfo) -> crate::api::deposit::Result<Self::Item<'a>> {
        cast(info, Self::offset(info), Self::size(info))
    }
    fn from_account_mut<'a>(
        info: &'a AccountInfo,
    ) -> crate::api::deposit::Result<Self::ItemMut<'a>> {
        cast_mut(info, Self::offset(info), Self::size(info))
    }
    fn size(_info: &AccountInfo) -> usize {
        size_of::<Self>()
    }
    fn offset(_info: &AccountInfo) -> usize {
        0
    }
}
