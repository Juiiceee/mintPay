use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use mpl_core::{
    instructions::{CreateCollectionV1CpiBuilder, CreateV1CpiBuilder},
    types::DataState,
};

use crate::states::Collection;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct MintCollection<'info> {
    #[account(init, payer = user, seeds = [b"collection", name.as_bytes(), user.key().as_ref()], bump, space = Collection::INIT_SPACE)]
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

impl<'info> MintCollection<'info> {
    pub fn initialize_collection(&mut self, name: String, uri: String, price: u64) -> Result<()> {
        CreateCollectionV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .payer(&self.user.to_account_info())
            .system_program(&self.system_program.to_account_info())
            .name(name.clone())
            .uri(uri.clone())
            .invoke()?;
        self.collection_account.price = price;
        self.collection_account.name = name;
        self.collection_account.uri = uri;

        // Stocker la clé publique de la collection dans notre compte
        self.collection_account.collection_address = self.collection.key();

        Ok(())
    }
}
