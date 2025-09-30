pub mod accounts;
pub mod api;
pub mod config;
mod entrypoint;
pub mod error;
pub mod executor;
mod State;

use api::*;

entrypoint! {
    Create => create_account,
    Resize => resize_account,
    Transfer => transfer,
    TransferFrom => transfer_from,
    Alloc => allocate_account,
    Assign => assign_account,
    Deposit => deposit,
}
