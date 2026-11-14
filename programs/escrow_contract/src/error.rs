use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid Maker")]
    InvalidMaker,
    #[msg("Invalid mint a")]
    InvalidMintA,
    #[msg("Invalid mint b")]
    InvalidMintB,
}
