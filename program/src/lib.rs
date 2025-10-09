pub mod accounts;
pub mod api;
pub mod config;
pub mod entrypoint;
pub mod error;
pub mod state;

pub use {api::*, instruction::*, state::*};

entrypoint! {
    Create => create_account,
    Resize => resize_account,
    Transfer => transfer,
    TransferFrom => transfer_from,
    Alloc => allocate_account,
    Assign => assign_account,
    Deposit => deposit,
}
