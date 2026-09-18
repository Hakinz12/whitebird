use anchor_lang::prelude::*;

#[error_code]
pub enum GuardError {
    #[msg("Transfer blocked: wallet is not whitelisted and token is still locked.")]
    TransferNotAllowed,
    #[msg("Unauthorized.")]
    Unauthorized,
    #[msg("Unlock timestamp must be in the future.")]
    InvalidTimestamp,
}
