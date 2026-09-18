use anchor_lang::prelude::*;
use crate::constants::LOCK_CONFIG_SEED;
use crate::errors::GuardError;
use crate::state::LockConfig;

#[derive(Accounts)]
pub struct InitializeLockConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: stored only
    pub mint: UncheckedAccount<'info>,
    #[account(init, payer = admin, space = 8 + LockConfig::INIT_SPACE, seeds = [LOCK_CONFIG_SEED, mint.key().as_ref()], bump)]
    pub lock_config: Account<'info, LockConfig>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeLockConfig>, unlock_timestamp: i64) -> Result<()> {
    let clock = Clock::get()?;
    require!(unlock_timestamp > clock.unix_timestamp, GuardError::InvalidTimestamp);
    let lc = &mut ctx.accounts.lock_config;
    lc.mint = ctx.accounts.mint.key();
    lc.admin = ctx.accounts.admin.key();
    lc.unlock_timestamp = unlock_timestamp;
    lc.bump = ctx.bumps.lock_config;
    Ok(())
}
