use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LockConfig {
    pub mint: Pubkey,
    pub admin: Pubkey,
    pub unlock_timestamp: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct WhitelistEntry {
    pub mint: Pubkey,
    pub wallet: Pubkey,
    pub bump: u8,
}
