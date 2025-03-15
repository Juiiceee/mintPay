use anchor_lang::prelude::*;

mod instructions;
mod states;

use instructions::*;

declare_id!("4KdNoKt4r4cxqZx2nLJ2Tg6B8nJdwRdi9N8ijKxh5sbg");

#[program]
pub mod mint_pay {
    use super::*;

    pub fn initialize_mint(ctx: Context<MintAsset>, name: String, uri: String) -> Result<()> {
        // Access the admin bump directly from ctx.bumps
        let admin_bump = ctx.bumps.admin;
        ctx.accounts.initialize_mint(name, uri, admin_bump)
    }

    pub fn initialize_collection(
        ctx: Context<MintCollection>,
        name: String,
        uri: String,
    ) -> Result<()> {
        let admin_bump = ctx.bumps.admin;
        ctx.accounts.initialize_collection(name, uri, admin_bump)
    }
}

#[derive(Accounts)]
pub struct TestIds<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    pub mpl_core_program: UncheckedAccount<'info>,
}
