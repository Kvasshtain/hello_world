pub mod api;
pub mod executor;
use executor::execute;

// entrypoint of the contract
solana_program_entrypoint::entrypoint!(execute);
