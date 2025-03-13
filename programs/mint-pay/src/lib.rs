use anchor_lang::prelude::*;

mod states;
mod instructions;

use instructions::*;

declare_id!("BVFkXCp3dK1RxBC9MpgEsaBCLw2AHeRyb3rwSH3irjMz");

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
        ctx.accounts.initialize_collection(name, uri, price)
    }
}

