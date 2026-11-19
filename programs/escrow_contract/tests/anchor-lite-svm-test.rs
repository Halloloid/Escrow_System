#![allow(unexpected_cfgs)]

use anchor_lang::{self, system_program};
use anchor_litesvm::{AnchorLiteSVM, Signer};
use litesvm_utils::{AssertionHelpers, TestHelpers};
use spl_associated_token_account::get_associated_token_address;
use spl_token;

anchor_lang::declare_program!(escrow_contract);

#[test]
fn test_escrow_make_and_take() {
    let program_id = escrow_contract::ID;

    let mut ctx = AnchorLiteSVM::build_with_program(
        program_id,
        include_bytes!("../../../target/deploy/escrow_contract.so"),
    );

    let maker = ctx.svm.create_funded_account(10_000_000_000).unwrap(); // 10 SOL
    let taker = ctx.svm.create_funded_account(10_000_000_000).unwrap(); // 10 SOL

    let mint_a = ctx.svm.create_token_mint(&maker, 9).unwrap();
    let mint_b = ctx.svm.create_token_mint(&maker, 9).unwrap();

    // Maker Account for Mint A
    let maker_ata_a = ctx
        .svm
        .create_associated_token_account(&mint_a.pubkey(), &maker)
        .unwrap();
    ctx.svm
        .mint_to(&mint_a.pubkey(), &maker_ata_a, &maker, 1_000_000_000)
        .unwrap(); // 1.0 tokens

    // Taker Account for Mint B
    let taker_ata_b = ctx
        .svm
        .create_associated_token_account(&mint_b.pubkey(), &taker)
        .unwrap();
    ctx.svm
        .mint_to(&mint_b.pubkey(), &taker_ata_b, &taker, 500_000_000)
        .unwrap(); // 0.5 tokens

    let seed: u64 = 42;
    let escrow_pda = ctx.svm.get_pda(
        &[b"escrow", maker.pubkey().as_ref(), &seed.to_le_bytes()],
        &program_id,
    );
    let vault = get_associated_token_address(&escrow_pda, &mint_a.pubkey());

    let maker_ix = ctx
        .program()
        .accounts(escrow_contract::client::accounts::Make {
            maker: maker.pubkey(),
            escrow: escrow_pda,
            mint_a: mint_a.pubkey(),
            mint_b: mint_b.pubkey(),
            maker_ata_a,
            vault,
            associated_token_program: spl_associated_token_account::id(),
            token_program: spl_token::id(),
            system_program: system_program::ID,
        })
        .args(escrow_contract::client::args::Make {
            seed,
            receive: 500_000_000,
            amount: 1_000_000_000,
        })
        .instruction()
        .unwrap();

    ctx.execute_instruction(maker_ix, &[&maker])
        .unwrap()
        .assert_success();

    assert!(
        ctx.account_exists(&escrow_pda),
        "Escrow Account Should Exist"
    );
    ctx.svm.assert_token_balance(&vault, 1_000_000_000);
    ctx.svm.assert_token_balance(&maker_ata_a, 0);

    let taker_ata_a = get_associated_token_address(&taker.pubkey(), &mint_a.pubkey());
    let maker_ata_b = get_associated_token_address(&maker.pubkey(), &mint_b.pubkey());

    let taker_ix = ctx
        .program()
        .accounts(escrow_contract::client::accounts::Take {
            taker: taker.pubkey(),
            maker: maker.pubkey(),
            escrow: escrow_pda,
            mint_a: mint_a.pubkey(),
            mint_b: mint_b.pubkey(),
            vault,
            taker_ata_a,
            taker_ata_b,
            maker_ata_b,
            associated_token_program: spl_associated_token_account::id(),
            token_program: spl_token::id(),
            system_program: system_program::ID,
        })
        .args(escrow_contract::client::args::Take {})
        .instruction()
        .unwrap();

    ctx.execute_instruction(taker_ix, &[&taker])
        .unwrap()
        .assert_success();

    // Need to Verify Accounts Were Closed
    ctx.svm.assert_account_closed(&escrow_pda);
    ctx.svm.assert_account_closed(&vault);

    // Need to Verify The Account After The Swap
    ctx.svm.assert_token_balance(&taker_ata_a, 1_000_000_000);
    ctx.svm.assert_token_balance(&taker_ata_b, 0);
    ctx.svm.assert_token_balance(&maker_ata_b, 500_000_000);
}
