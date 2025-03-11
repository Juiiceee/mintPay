use anchor_lang::prelude::*;

use mpl_core::{instructions::CreateV1CpiBuilder, types::DataState};

declare_id!("P4cYdsn1RYax3nA8Pmw54rpraBxKUmPU6PU1ZQC39jA");

#[program]
pub mod mint_pay {
    use super::*;

    pub fn initialize_mint(ctx: Context<MintAsset>, name: String, uri: String) -> Result<()> {
        CreateV1CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.mint.to_account_info())
            .collection(None)
            .payer(&ctx.accounts.user.to_account_info())
            .update_authority(None)
            .system_program(&ctx.accounts.system_program.to_account_info())
            .data_state(DataState::AccountState)
            .name(name)
            .uri(uri)
            .invoke()?;

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
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}
