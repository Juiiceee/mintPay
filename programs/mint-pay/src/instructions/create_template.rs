use anchor_lang::prelude::*;

use crate::states::template::*;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateTemplate<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 25 + 200 + 32 + 200 + (1 + 1000 + 8) + 4 + (10 * (32 + 200 + 1 + 32)), 
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
		description: String,
		image: String,
		attribut: Attribut,
		parameter: Parameter) -> Result<()> {
		self.template.name = name;
		self.template.description = description;
		self.template.creator = self.user.key();
		self.template.image = image;
		self.template.attributs = attribut;
		self.template.parameters.push(parameter);
        Ok(())
    }
}
