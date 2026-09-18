use anchor_lang::prelude::*;
use crate::constants::{LOCK_CONFIG_SEED, WHITELIST_SEED};
use crate::errors::GuardError;
use crate::state::{LockConfig, WhitelistEntry};

#[derive(Accounts)]
#[instruction(wallet: Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: mint for PDA seeds
    pub mint: UncheckedAccount<'info>,
    #[account(seeds = [LOCK_CONFIG_SEED, mint.key().as_ref()], bump = lock_config.bump, has_one = admin @ GuardError::Unauthorized)]
    pub lock_config: Account<'info, LockConfig>,
    #[account(init, payer = admin, space = 8 + WhitelistEntry::INIT_SPACE, seeds = [WHITELIST_SEED, mint.key().as_ref(), wallet.as_ref()], bump)]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddToWhitelist>, wallet: Pubkey) -> Result<()> {
    let e = &mut ctx.accounts.whitelist_entry;
    e.mint = ctx.accounts.mint.key();
    e.wallet = wallet;
    e.bump = ctx.bumps.whitelist_entry;
    Ok(())
}
