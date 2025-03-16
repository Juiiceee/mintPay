use anchor_lang::prelude::*;

use crate::states::Template;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateTemplate<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 25 + 200 + 32 + 8, 
        seeds = [b"template", name.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub template: Account<'info, Template>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateTemplate<'info> {
    pub fn create_template(&mut self, name: String, 
		uri: String,
		price: u64) -> Result<()> {
		self.template.name = name;
		self.template.creator = self.user.key();
		self.template.uri = uri;
		self.template.price = price;
		Ok(())
	}
}
