use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ProgramError {
    #[error("Invalid Version")]
    InvalidVersion,
    #[error("Invalid PDA")]
    InvalidPDA,
}

impl From<ProgramError> for solana_program::program_error::ProgramError {
    fn from(value: ProgramError) -> Self {
        solana_program::program_error::ProgramError::Custom(value as u32)
    }
}
