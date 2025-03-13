use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

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
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> MintAsset<'info> {
    pub fn initialize_mint(&self) -> Result<()> {
        if self.collection.to_account_info().owner.ne(&self.user.key()) {
            let transfer_instruction = Transfer {
                from: self.user.to_account_info(),
                to: self.recipient.to_account_info(),
            };
            let cpi_ctx =
                CpiContext::new(self.system_program.to_account_info(), transfer_instruction);

            system_program::transfer(cpi_ctx, self.collection.price)?;
        }

        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .asset(&self.mint.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .payer(&self.user.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .data_state(DataState::AccountState)
            .name(self.collection.name.clone())
            .uri(self.collection.uri.clone())
            .invoke()?;

        Ok(())
    }
}
