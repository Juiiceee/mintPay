use anchor_lang::prelude::*;

mod instructions;
mod states;

use instructions::*;
use states::*;

declare_id!("6MZX72JXyargbjb3Cgx5HzgS5awKdRbciCx3LPHMvhpn");

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

    pub fn create_template(
        ctx: Context<CreateTemplate>,
        name: String,
        description: String,
        image: String,
        attribut: Attribut,
        parameter: Parameter,
    ) -> Result<()> {
        ctx.accounts.create_template(name, description, image, attribut, parameter)
    }
}
