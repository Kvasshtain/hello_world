use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        account_info::{Account as AccountTrait, AccountInfo},
        clock::Epoch,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
};
use crate::error::Error;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Account {
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: Epoch,
    pub writable: bool,
}

impl Account {
    pub fn from_account_info(info: &AccountInfo) -> Self {
        Self {
            lamports: info.lamports(),
            data: info.data.borrow().to_vec(),
            owner: *info.owner,
            executable: info.executable,
            rent_epoch: info.rent_epoch,
            writable: info.is_writable,
        }
    }

    pub fn to_account_info(&self, info: &AccountInfo) -> Result<(), Error> {
        {
            let mut lamports_ref = info.lamports.borrow_mut();
            **lamports_ref = self.lamports;
        }

        {
            let mut data_ref = info.data.borrow_mut();

            if data_ref.len() != self.data.len() {
                return Err(Error::AccountDataMismatch);
            }

            data_ref.copy_from_slice(&self.data);

            Ok(())

            // let data_slice: &mut [u8] = &mut *data_ref;
            // assert_eq!(
            //     data_slice.len(),
            //     self.data.len(),
            //     "destination account data length does not match source"
            // );
            // data_slice.copy_from_slice(&self.data);
        }
    }
}