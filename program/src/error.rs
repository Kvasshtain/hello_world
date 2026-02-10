use {solana_program_error::ProgramError, solana_pubkey::Pubkey, thiserror::Error};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid data length: {0} {1}, {2}")]
    InvalidDataLength(Pubkey, usize, usize),

    #[error("Calculation overflow")]
    CalculationOverflow,

    #[error("Calculation underflow")]
    CalculationUnderflow,

    #[error("account not found: {0}")]
    AccountNotFound(Pubkey),

    #[error("Signer not found, or more than one signer was found")]
    InvalidSigner,

    #[error("Account is not correct")]
    WrongAccount,

    #[error("Wrong index")]
    WrongIndex,

    #[error("A solana program error: {0}")]
    ProgramError(ProgramError),

    #[error("Insufficient account balance")]
    InsufficientBalance,

    #[error("Account is locked")]
    AccountLocked,

    #[error("The AccountInfo has an invalid owner: {0}")]
    InvalidOwner(Pubkey),

    #[error("attempt to init an initialized account: {0}")]
    AccountInitialized(Pubkey),

    #[error("Invalid account type: {0}")]
    InvalidAccountType(Pubkey),

    #[error("Index out of range")]
    IndexOutOfRange,

    #[error("List is empty")]
    ListIsEmpty,

    #[error("Invalid serialized data")]
    InvalidData,
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
