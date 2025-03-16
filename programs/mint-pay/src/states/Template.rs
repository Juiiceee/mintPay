use anchor_lang::prelude::*;

#[account]
pub struct Template {
    pub name: String,
    pub uri: String,
    pub creator: Pubkey,
    pub price: u64,
}