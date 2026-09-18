use anchor_lang::prelude::*;
use crate::constants::{LOCK_CONFIG_SEED, WHITELIST_SEED};
use crate::errors::GuardError;
use crate::state::{LockConfig, WhitelistEntry};

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: mint
    pub mint: UncheckedAccount<'info>,
    #[account(seeds = [LOCK_CONFIG_SEED, mint.key().as_ref()], bump = lock_config.bump, has_one = admin @ GuardError::Unauthorized)]
    pub lock_config: Account<'info, LockConfig>,
    #[account(mut, close = admin, seeds = [WHITELIST_SEED, mint.key().as_ref(), wallet.as_ref()], bump = whitelist_entry.bump)]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<RemoveFromWhitelist>, _wallet: Pubkey) -> Result<()> { Ok(()) }
