pub mod accounts;
pub mod api;
pub mod config;
mod entrypoint;
pub mod error;
pub mod executor;
mod State;

use api::*;

entrypoint! {
    Alloc => allocate_account,
    Assign => assign_account,
    Create => create_account,
    Deposit => deposit,
    Resize => resize_account,
    Transfer => transfer,
    TransferFrom => transfer_from,
}
