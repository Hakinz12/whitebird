use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("771om81G5Ey5z1M6cb5NEu6s7dZmapUY3zwBgR4qU6Z9");

#[program]
pub mod token_guard {
    use super::*;

    pub fn initialize_lock_config(ctx: Context<InitializeLockConfig>, unlock_timestamp: i64) -> Result<()> {
        instructions::initialize_lock_config::handler(ctx, unlock_timestamp)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, wallet: Pubkey) -> Result<()> {
        instructions::add_to_whitelist::handler(ctx, wallet)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, wallet: Pubkey) -> Result<()> {
        instructions::remove_from_whitelist::handler(ctx, wallet)
    }

    pub fn initialize_extra_account_meta_list(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        instructions::initialize_extra_account_meta_list::handler(ctx)
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        instructions::execute::handler(ctx, amount)
    }
}
