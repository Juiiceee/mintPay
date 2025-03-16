use anchor_lang::prelude::*;

mod instructions;
mod states;

use instructions::*;

declare_id!("HueUgKCWC6Q6cj7g9dWLixhwE4uXH7CGBF4pn9ugXWWr");

#[program]
pub mod mint_pay {
    use super::*;

    pub fn initialize_mint(ctx: Context<MintAsset>) -> Result<()> {
        let admin_bump = ctx.bumps.admin;
        ctx.accounts.initialize_mint(admin_bump)
    }

    pub fn initialize_collection(
        ctx: Context<MintCollection>,
        name: String,
        uri: String,
    ) -> Result<()> {
        let admin_bump = ctx.bumps.admin;
        ctx.accounts.initialize_collection(name, uri, admin_bump)
    }

    pub fn create_template(
        ctx: Context<CreateTemplate>,
        name: String,
        uri: String,
        price: u64,
    ) -> Result<()> {
        ctx.accounts.create_template(name, uri, price)
    }
}
