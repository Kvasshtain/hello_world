use std::collections::BTreeMap;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use {
    crate::{
        accounts::Data,
        error::Error,
    },
    solana_program::account_info::AccountInfo,
    std::{
        cell::{Ref, RefMut},
    },
};
use crate::accounts::{cast_slice, cast_slice_mut};
use crate::accounts::account::Account;
use crate::accounts::account_lock::AccountLock;
use crate::accounts::account_state::AccountState;
use crate::accounts::account_type::AccountType;
use crate::accounts::holder_lock::HolderLock;
use crate::cast_data_slice;
use crate::error::Error::CalculationOverflow;

#[repr(C, packed)]
pub struct HolderData {
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HolderMap {
    pub map: BTreeMap<Pubkey, Account>,
}

impl HolderData {
    pub fn init(info: &AccountInfo) -> Result<(), Error> {
        AccountLock::init(info, AccountType::StateHolder)?;
    
        Ok(())
    }
    
    pub fn get(holder_info: &AccountInfo, key: Pubkey) -> Result<Account, Error> {
        let holder = HolderData::from_account_mut(holder_info)?;

        let decoded_state = HolderMap::try_from_slice(&*holder)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        let account = decoded_state.map.get(&key).ok_or(Error::AccountNotFound(key))?;

        Ok(account.clone())
    }

    pub fn map(holder_info: &AccountInfo) -> Result<HolderMap, Error> {
        let holder = HolderData::from_account_mut(holder_info)?;

        let holder_map = HolderMap::try_from_slice(&*holder)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(holder_map)
    }

    pub fn set(holder_info: &AccountInfo, key: Pubkey, balance_info: &AccountInfo) -> Result<(), Error> {
        let balance_account = Account::from_account_info(balance_info);

        HolderData::set_account(holder_info, key, balance_account)?;

        Ok(())
    }

    pub fn set_account(holder_info: &AccountInfo, key: Pubkey, balance_account: Account) -> Result<(), Error> {
        let mut holder = HolderData::from_account_mut(holder_info)?;

        // let mut decoded_state = HolderMap::try_from_slice(&*holder)
        //     .map_err(|_| ProgramError::InvalidAccountData)?;
        
        let mut decoded_state = HolderData::map(holder_info)?;

        decoded_state.map.insert(key, balance_account);

        let mut writer = &mut holder[..];
        decoded_state.serialize(&mut writer)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        Ok(())
    }
    
    pub fn update_balance(holder: &AccountInfo, new_amount: u64, account: &Account, balance_pda_key: Pubkey) -> Result<(), Error> {
        let state = cast_data_slice::<AccountState>(&account.data);

        let new_state = AccountState {
            balance: state.balance.checked_sub(new_amount)
                .ok_or(CalculationOverflow)?,
        };

        let new_account = Account {
            lamports: account.lamports,
            data: new_state.serialize(),
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
            writable: account.writable,
        };

        HolderData::set_account(holder, balance_pda_key, new_account)?;

        Ok(())
    }
}

impl Data for HolderData {
    type Item<'a> = Ref<'a, [u8]>;
    type ItemMut<'a> = RefMut<'a, [u8]>;

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
