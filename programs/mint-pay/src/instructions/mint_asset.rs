use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use mpl_core::{
    instructions::CreateV1CpiBuilder,
    types::DataState,
};
use std::str::FromStr;

use crate::states::Template;

#[derive(Accounts)]
pub struct MintAsset<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the mint account of the asset to be minted
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
	pub template: Account<'info, Template>,
    /// CHECK: This is the Metaplex collection account
    #[account(mut)]
    pub metaplex_collection: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"admin"],
        bump,
        seeds::program = crate::id(),
    )]
    /// CHECK: This account is a PDA used as a signer
    pub admin: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap())]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> MintAsset<'info> {
    pub fn initialize_mint(&self, admin_bump: u8) -> Result<()> {
        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .payer(&self.user.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .asset(&self.mint.to_account_info())
            .collection(Some(&self.metaplex_collection.to_account_info()))
            .authority(Some(&self.admin.to_account_info())) // Gardez l'autorité pour signer
            .owner(Some(&self.user.to_account_info())) // Le propriétaire sera l'utilisateur
            .data_state(DataState::AccountState)
            .name(self.template.name.clone())
            .uri(self.template.uri.clone())
            .invoke_signed(&[&[b"admin", &[admin_bump]]])?;

        Ok(())
    }
}
