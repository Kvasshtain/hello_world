use {
    super::{cast, cast_mut, Data, Result},
    crate::{
        accounts::{
            account_type::AccountType::self,
            ver::Ver,
        },
        error::Error,
    },
    solana_program::{account_info::AccountInfo, pubkey::Pubkey},
    std::{
        cell::{Ref, RefMut},
        fmt::Debug,
        mem::size_of,
    },
};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AccountLock {
    pub holder: Pubkey,
}

impl AccountLock {
    pub fn init(info: &AccountInfo, typ: AccountType) -> std::result::Result<(), Error> {
        Ver::init(info, typ)?;

        let mut lock = AccountLock::from_account_mut(info)?;
        *lock = AccountLock {
            holder: Pubkey::default(),
        };

        Ok(())
    }
    
    pub fn get(&self) -> Result<bool> {
        // !!!!!!!!!!!Доролни проверкой на протухание в холдере!!!!

        Ok(self.holder != Pubkey::default())
    }

    pub fn lock(&mut self, holder: &Pubkey) -> std::result::Result<(), Error> {
        self.holder = *holder;
        Ok(())
    }

    pub fn unlock(&mut self) -> std::result::Result<(), Error> {
        self.holder = Pubkey::default();
        Ok(())
    }
}

impl Data for AccountLock {
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
