use {
    anyhow::Result,
    crate::accounts::{cast, Data},
    std::cell::Ref,
    std::mem::size_of,
};

#[repr(C, packed)]
pub struct AccountState {
    pub(crate) balance: u64,
}

impl Data for AccountState {
    type Item<'a> = Ref<'a, Self>;

    async fn from_arr<'a>(data: Ref<'a, &[u8]>) -> Result<Self::Item<'a>> {
        cast(data, 0, size_of::<Self>()).await
    }
}
