use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey, // the person who creates the escrow
    pub mint_a: Pubkey, // mint account for token 1
    pub mint_b: Pubkey, // mint account for token 2
    pub receive: u64,  // the person going to recive the funds
    pub bumb: u8,
}
