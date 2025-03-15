use anchor_lang::prelude::*;

mod instructions;
mod states;

use instructions::*;

declare_id!("DoKegcFEw8xNS5uy8nein8SoPPpzpRoGau45zsB7jcDx");

#[program]
pub mod mint_pay {
    use super::*;

    pub fn initialize_mint(ctx: Context<MintAsset>) -> Result<()> {
        // Access the admin bump directly from ctx.bumps
        let admin_bump = ctx.bumps.admin;
        ctx.accounts.initialize_mint(admin_bump)
    }

    pub fn initialize_collection(
        ctx: Context<MintCollection>,
        name: String,
        uri: String,
        price: u64,
    ) -> Result<()> {
        // Access the admin bump directly from ctx.bumps
        let admin_bump = ctx.bumps.admin;
        ctx.accounts
            .initialize_collection(name, uri, price, admin_bump)
    }

    pub fn test_ids(ctx: Context<TestIds>) -> Result<()> {
        msg!("System Program ID: {}", ctx.accounts.system_program.key());
        msg!(
            "MPL Core Program ID: {}",
            ctx.accounts.mpl_core_program.key()
        );
        msg!("mpl_core::ID: {}", mpl_core::ID);
        Ok(())
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
