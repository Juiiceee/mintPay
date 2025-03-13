use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Collection {
    pub collection_address: Pubkey,
    pub price: u64,
    #[max_len(30)]
    pub name: String,
    #[max_len(150)]
    pub uri: String,
}

