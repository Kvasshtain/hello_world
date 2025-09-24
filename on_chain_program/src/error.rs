use {solana_program_error::ProgramError, solana_pubkey::Pubkey, thiserror::Error};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid data length: {0} {1}, {2}")]
    InvalidDataLength(Pubkey, usize, usize),

    #[error("Calculation overflow")]
    CalculationOverflow,

    #[error("account not found: {0}")]
    AccountNotFound(Pubkey),
}

impl From<Error> for ProgramError {
    fn from(_err: Error) -> ProgramError {
        ProgramError::Custom(0)
    }
}
