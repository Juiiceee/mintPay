use anchor_lang::prelude::*;

#[account]
pub struct Template {
    pub name: String,
    pub description: String,
	pub creator: Pubkey,
    pub image: String,
    pub attributs: Attribut,
    pub parameters: Vec<Parameter>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Attribut {
    pub number_of_parameters: u8,
    pub execution_code: String,
    pub price: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Parameter {
    pub name_parameter: String,
    pub description_parameter: String,
    pub required: bool,
    pub style: String,
}