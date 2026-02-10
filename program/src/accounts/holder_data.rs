use {
    crate::{
        accounts::{cast, cast_mut, Data},
        error::Error,
    },
    solana_program::account_info::AccountInfo,
    solana_pubkey::Pubkey,
    std::{
        cell::{Ref, RefMut},
        collections::HashMap,
        mem::size_of,
    },
};
use crate::accounts::{cast_slice, cast_slice_mut};
use crate::accounts::account_lock::AccountLock;
use crate::accounts::account_type::AccountType;
use crate::accounts::holder_lock::HolderLock;
use crate::error::Error::WrongIndex;

#[repr(C, packed)]
pub struct HolderData {

}

impl HolderData {
    pub fn init(info: &AccountInfo) -> Result<(), Error> {
        AccountLock::init(info, AccountType::StateHolder)?;
    
        Ok(())
    }

    // pub fn add(&mut self, pda: &Pubkey, balance: u64) -> Result<(), Error> {
    //     //let balances = unsafe { &mut *std::ptr::addr_of_mut!(self.balances) };
    //     let mut balances = self.balances.clone();
    //     balances.push(balance);
    //
    //     Ok(())
    // }

    pub fn add_from(info: &AccountInfo,  balance: u64) -> Result<(), Error> {
        let mut holder = HolderData::from_account_mut(info)?;
        let mut vec = holder.to_vec();

        if(vec.len() > 0) {
            return Ok(()); // from (signer) has already been added
        }

        vec.push(balance);
        holder.copy_from_slice(&*vec);
        Ok(())
    }

    pub fn add_to(info: &AccountInfo, balance: u64) -> Result<usize, Error> {
        let mut holder = HolderData::from_account_mut(info)?;

        let mut vec = holder.to_vec();

        if(vec.len() == 0) {
            return Err(Error::ListIsEmpty); // from (signer) hasn't already been added. Add from (signer) first
        }

        vec.push(balance);
        let index = vec.len() - 1;
        holder.copy_from_slice(&*vec);
        Ok(index)
    }

    // pub fn remove(info: &AccountInfo,  index: usize) -> Result<(), Error> {
    //     let mut holder = HolderData::from_account_mut(info)?;
    // 
    //     let mut vec = holder.to_vec();
    //     vec.remove(index);
    //     holder.copy_from_slice(&*vec);
    //     Ok(())
    // }

    pub fn get(info: &AccountInfo, index: usize) -> Result<u64, Error> {

        if !HolderData::index_exist(info, index)? {
            return Err(Error::IndexOutOfRange);
        }
        
        let data = HolderData::from_account_mut(info)?;

        Ok(data[index])
    }

    pub fn index_exist(info: &AccountInfo, index: usize) -> Result<bool, Error> {
        let data = HolderData::from_account_mut(info)?;

        if index >= data.len() {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn set(info: &AccountInfo, index: usize, balance: u64) -> Result<(), Error> {
        let mut holder = HolderData::from_account_mut(info)?;
        holder[index] = balance;
        Ok(())
    }
}

impl Data for HolderData {
    type Item<'a> = Ref<'a, [u64]>;
    type ItemMut<'a> = RefMut<'a, [u64]>;

    fn from_account<'a>(info: &'a AccountInfo) -> Result<Self::Item<'a>, Error> {
        cast_slice(info, Self::offset(info), Self::size(info))
    }

    fn from_account_mut<'a>(info: &'a AccountInfo) -> Result<Self::ItemMut<'a>, Error> {
        cast_slice_mut(info, Self::offset(info), Self::size(info))
    }

    fn size(info: &AccountInfo) -> usize {
        assert!(info.data_len() >= Self::offset(info));
        info.data_len() - Self::offset(info)
    }
    fn offset(info: &AccountInfo) -> usize {
        HolderLock::offset(info) + HolderLock::size(info)
    }
}
