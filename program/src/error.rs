use {solana_program_error::ProgramError, solana_pubkey::Pubkey, thiserror::Error};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid data length: {0} {1}, {2}")]
    InvalidDataLength(Pubkey, usize, usize),

    #[error("Calculation overflow")]
    CalculationOverflow,

    #[error("account not found: {0}")]
    AccountNotFound(Pubkey),

    #[error("Signer not found, or more than one signer was found")]
    InvalidSigner,

    #[error("Account is already owned")]
    AlreadyOwned,

    #[error("An solana program error: {0}")]
    ProgramError(ProgramError),
}

impl From<ProgramError> for Error {
    fn from(e: ProgramError) -> Self {
        Error::ProgramError(e)
    }
}

impl From<Error> for ProgramError {
    fn from(_err: Error) -> ProgramError {
        ProgramError::Custom(0)
    }
}
