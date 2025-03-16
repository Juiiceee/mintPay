use anchor_lang::prelude::*;

use mpl_core::instructions::CreateCollectionV1CpiBuilder;

use crate::states::Collection;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct MintCollection<'info> {
    #[account(init, payer = user, seeds = [b"collection"], bump, space = 8 + 32)]
    pub collection_account: Account<'info, Collection>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"admin"],
        bump,
        seeds::program = crate::id(),
    )]
    /// CHECK: Ce compte est un PDA utilisé comme signataire
    pub admin: UncheckedAccount<'info>,
    #[account(mut)]
    pub collection: Signer<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is the ID of the Metaplex Core program
    #[account(address = mpl_core::ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

impl<'info> MintCollection<'info> {
    pub fn initialize_collection(
        &mut self,
        name: String,
        uri: String,
        admin_bump: u8,
    ) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .update_authority(Some(&self.admin.to_account_info()))
            .payer(&self.user.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name(name.clone())
            .uri(uri.clone())
            .invoke_signed(&[&[b"admin", &[admin_bump]]])?;
        self.collection_account.collection_address = self.collection.key();

        Ok(())
    }
}
