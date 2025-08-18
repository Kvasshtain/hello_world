pub mod executor;
pub mod api;
use executor::execute;


// entrypoint of the contract
solana_program_entrypoint::entrypoint!(execute);

