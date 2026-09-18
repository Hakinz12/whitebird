use anchor_lang::prelude::*;
use spl_transfer_hook_interface::instruction::ExecuteInstruction;
use spl_tlv_account_resolution::{account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList};
use crate::constants::{LOCK_CONFIG_SEED, WHITELIST_SEED, EXTRA_ACCOUNT_META_LIST_SEED};

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: mint
    pub mint: UncheckedAccount<'info>,
    /// CHECK: PDA
    #[account(init, payer = payer, space = ExtraAccountMetaList::size_of(2).unwrap(), seeds = [EXTRA_ACCOUNT_META_LIST_SEED, mint.key().as_ref()], bump)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
    let metas = vec![
        ExtraAccountMeta::new_with_seeds(&[Seed::Literal { bytes: LOCK_CONFIG_SEED.to_vec() }, Seed::AccountKey { index: 1 }], false, true)?,
        ExtraAccountMeta::new_with_seeds(&[Seed::Literal { bytes: WHITELIST_SEED.to_vec() }, Seed::AccountKey { index: 1 }, Seed::AccountKey { index: 3 }], false, true)?,
    ];
    let mut data = ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?;
    ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &metas)?;
    Ok(())
}
