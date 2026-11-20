#![allow(unexpected_cfgs)]

pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("AhQt94qKBM9jMy8EMQ47QMkn56r7YVjDyiGipLYNiAH5");

#[program]
pub mod escrow_contract {
    use super::*;

    pub fn make(ctx:Context<Make>,seed:u64,receive:u64,amount:u64) -> Result<()>{
        instructions::make::handler(ctx, seed, receive, amount)
    }

    pub fn take(ctx:Context<Take>) -> Result<()>{
        instructions::take::handler(ctx)
    }

    pub fn refund(ctx:Context<Refund>) -> Result<()>{
        instructions::refund::handler(ctx)
    } 
}
