use anchor_lang::prelude::*;

#[error_code]
pub enum GuardError {
    #[msg("Failed Transfer")]
    TransferNotAllowed,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Unlock timestamp must be in the future.")]
    InvalidTimestamp,
}
