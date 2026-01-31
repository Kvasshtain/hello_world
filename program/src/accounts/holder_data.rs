use std::collections::HashMap;
use solana_pubkey::Pubkey;
use {
    crate::{
        accounts::{cast, cast_mut, Data},
        error::Error,
    },
    solana_program::account_info::AccountInfo,
    std::cell::{Ref, RefMut},
    std::mem::size_of,
};

#[repr(C, packed)]
pub struct HolderData {
    pub balances: HashMap<[u8; 32], u64>,
}

impl HolderData {
    pub fn init(info: &AccountInfo) -> Result<(), Error> {
        let mut state = HolderData::from_account_mut(info)?;

        *state = HolderData {
            balances: HashMap::new(),
        };

        Ok(())
    }

    pub fn add(&mut self, pda: &Pubkey, balance: u64) -> Result<(), Error> {
        let balances = unsafe { &mut *std::ptr::addr_of_mut!(self.balances) };
        balances.insert(pda.to_bytes(), balance);

        Ok(())
    }
}

impl Data for HolderData {
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

    fn offset(_info: &AccountInfo) -> usize {
        0
    }
}
