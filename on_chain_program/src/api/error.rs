use solana_program_error::ProgramError;
use {solana_pubkey::Pubkey, thiserror::Error};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid data length: {0} {1}, {2}")]
    InvalidDataLength(Pubkey, usize, usize),
    #[error("Calculation overflow")]
    CalculationOverflow,
}

impl From<Error> for ProgramError {
    fn from(err: Error) -> ProgramError {
        match err {
            _ => ProgramError::Custom(0),
        }
    }
}
