#![cfg(feature = "test-bpf")]

use borsh::BorshDeserialize;
use teachable02::UserStake;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::{
    signer::keypair::Keypair,
    system_program,
    system_transaction,
    signature::Signer, transaction::Transaction
};
use solana_validator::test_validator::*;

#[test]
fn test_validator_transaction() {
    solana_logger::setup_with_default("solana_program_runtime=debug");
    let program_id = Pubkey::new_unique();

    let (test_validator, payer) = TestValidatorGenesis::default()
        .add_program("teachable02", program_id)
        .start();
    let rpc_client = test_validator.get_rpc_client();

    let blockhash = rpc_client.get_latest_blockhash().unwrap();

    let alice = Keypair::new();
    // airdrop 1 SOL
    const ALICE_INIT_BALANCE: u64 = 1_000_000_000;
    {
        let tx =
            system_transaction::transfer(&payer, &alice.pubkey(), ALICE_INIT_BALANCE, blockhash);
        rpc_client.send_and_confirm_transaction(&tx).unwrap();
    }

    // Derive the PDA from the alice account, a string representing the unique
    // purpose of the account ("vault"), and the address of our on-chain program.
    let (alice_pda, _) = Pubkey::find_program_address(
        &[b"vault", alice.pubkey().as_ref()],
        &program_id);
    let instruction_data: Vec<u8> = Vec::new();
    let mut transaction = Transaction::new_with_payer(
        &[Instruction{
            program_id,
            accounts: vec![
                AccountMeta::new(alice.pubkey(), true),
                AccountMeta::new(alice_pda, false),
                AccountMeta::new(system_program::id(), false),
            ],
            data: instruction_data,
        }],
        Some(&alice.pubkey()),
    );
    transaction.sign(&[&alice], blockhash);

    rpc_client
        .send_and_confirm_transaction(&transaction)
        .unwrap();

    let account_data = rpc_client.get_account_data(&alice_pda).unwrap();
    let user_stake = UserStake::try_from_slice(&account_data).unwrap();

    assert!(user_stake.is_initialized);
    assert_eq!(user_stake.lamports, 42);
}

