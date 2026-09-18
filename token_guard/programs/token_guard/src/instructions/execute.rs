use anchor_lang::prelude::*;
use crate::constants::{LOCK_CONFIG_SEED, WHITELIST_SEED};
use crate::errors::GuardError;
use crate::state::LockConfig;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    /// CHECK: source token acct
    pub source_token: UncheckedAccount<'info>,
    /// CHECK: mint
    pub mint: UncheckedAccount<'info>,
    /// CHECK: dest token acct
    pub destination_token: UncheckedAccount<'info>,
    /// CHECK: source owner
    pub owner: UncheckedAccount<'info>,
    /// CHECK: extra account meta list (required by Token-2022)
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(seeds = [LOCK_CONFIG_SEED, mint.key().as_ref()], bump)]
    pub lock_config: Account<'info, LockConfig>,
    /// CHECK: whitelist PDA may not exist
    #[account(seeds = [WHITELIST_SEED, mint.key().as_ref(), owner.key().as_ref()], bump)]
    pub whitelist_entry: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let lc = &ctx.accounts.lock_config;
    if clock.unix_timestamp >= lc.unlock_timestamp { return Ok(()); }
    if ctx.accounts.owner.key() == lc.admin { return Ok(()); }
    if ctx.accounts.whitelist_entry.data_len() > 0 { return Ok(()); }
    Err(GuardError::TransferNotAllowed.into())
}
