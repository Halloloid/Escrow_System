#![allow(unexpected_cfgs)]

use anchor_lang::{self,system_program};
use anchor_litesvm::{AnchorLiteSVM,Signer};
use litesvm_utils::{AssertionHelpers,TestHelpers};
use spl_associated_token_account::get_associated_token_address;
use spl_token;

anchor_lang::declare_program!(escrow_contract);

#[test]
fn test_escrow_make_and_take(){
    let program_id = escrow_contract::ID; 
    
}