pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use state::*;

declare_id!("AKX7pbPi3HdQW4TT5AYdXM6f68ALKh5fZ5i4xHuXiFT8");

#[program]
pub mod escrow_contract {
    use super::*;
}
