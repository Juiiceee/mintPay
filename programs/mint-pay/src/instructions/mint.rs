use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};
use std::str::FromStr;
use mpl_core::{
    instructions::{CreateCollectionV1CpiBuilder, CreateV1CpiBuilder},
    types::DataState,
};

use crate::states::Collection;

#[derive(Accounts)]
pub struct MintAsset<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// CHECK: This is the mint account of the asset to be minted
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    #[account(mut)]
    pub collection: Account<'info, Collection>,
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
        msg!("Starting initialize_mint");
        
        // Store all account references first to extend their lifetime
		let mpl_core_program_info = &self.mpl_core_program.to_account_info();
		let user_info = &self.user.to_account_info();
		let system_program_info = &self.system_program.to_account_info();
		let mint_info = &self.mint.to_account_info();
		let metaplex_collection_info = &self.metaplex_collection.to_account_info();
		let admin_info = &self.admin.to_account_info();
		let name = self.collection.name.clone();
		let uri = self.collection.uri.clone();
		
		msg!("Creating builder");
		
		// SUPPRIMEZ l'update_authority quand vous spécifiez une collection
		CreateV1CpiBuilder::new(mpl_core_program_info)
			.payer(user_info)
			.system_program(system_program_info)
			.asset(mint_info)
			.collection(Some(metaplex_collection_info))
			.authority(Some(admin_info))  // Gardez l'autorité pour signer
			.owner(Some(user_info))  // Le propriétaire sera l'utilisateur
			// .update_authority(admin_info)  // RETIREZ CETTE LIGNE
			.data_state(DataState::AccountState)
			.name(name)
			.uri(uri)
			.invoke_signed(&[&[b"admin", &[admin_bump]]])?;
		
		msg!("CPI executed successfully");
		
		Ok(())
    }
}
