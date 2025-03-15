use anchor_lang::prelude::*;

mod states;
mod instructions;

use instructions::*;

declare_id!("DoKegcFEw8xNS5uy8nein8SoPPpzpRoGau45zsB7jcDx");

#[program]
pub mod mint_pay {
    use super::*;

    pub fn initialize_mint(ctx: Context<MintAsset>) -> Result<()> {
        ctx.accounts.initialize_mint()
    }

    pub fn initialize_collection(
        ctx: Context<MintCollection>,
        name: String,
        uri: String,
        price: u64,
    ) -> Result<()> {
        // Access the admin bump directly from ctx.bumps
        let admin_bump = ctx.bumps.admin;
        ctx.accounts.initialize_collection(name, uri, price, admin_bump)
    }
}
