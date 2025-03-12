use anchor_lang::prelude::*;

use mpl_core::{
    instructions::{CreateCollectionV1CpiBuilder, CreateV1CpiBuilder},
    types::DataState,
};

declare_id!("mqukZBPWH6K7JXX3V4VEJGJYFvEwpSMpT4y9DCweU3k");

#[program]
pub mod mint_pay {
    use super::*;

    pub fn initialize_mint(ctx: Context<MintAsset>, name: String, uri: String) -> Result<()> {
        CreateV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.mint.to_account_info())
            .collection(Some(&ctx.accounts.collection.to_account_info()))
            .payer(&ctx.accounts.user.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .data_state(DataState::AccountState)
            .name(name)
            .uri(uri)
            .invoke()?;

        Ok(())
    }

    pub fn initialize_collection(
        ctx: Context<MintCollection>,
        name: String,
        uri: String,
    ) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .collection(&ctx.accounts.collection.to_account_info())
            .payer(&ctx.accounts.user.to_account_info())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(name)
            .uri(uri)
            .invoke()?;

        // Stocker la clé publique de la collection dans notre compte
        ctx.accounts.collection_account.owner = ctx.accounts.collection.key();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintAsset<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the mint account of the asset to be minted
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: This is the collection account of the asset to be minted
    #[account(mut)]
    pub collection: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MintCollection<'info> {
    #[account(init, payer = user, seeds = [b"collection", user.key().as_ref()], bump, space = 8 + 32)]
    pub collection_account: Account<'info, Collection>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub collection: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

#[account]
pub struct Collection {
    pub owner: Pubkey,
}
