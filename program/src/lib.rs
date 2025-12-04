#![allow(unexpected_cfgs)]

pub mod accounts;
pub mod api;
mod base;
pub mod config;
pub mod entrypoint;
pub mod error;
mod seed;
pub mod state;

pub use {api::*, instruction::*, state::*};

entrypoint! {
    Create => create_account,
    Resize => resize_account,
    Transfer => native_transfer,
    TransferFrom => native_transfer_from,
    Alloc => allocate_account,
    Assign => assign_account,
    Deposit => deposit,
    Withdraw => withdraw,
    InternalTransfer => internal_transfer,
}
