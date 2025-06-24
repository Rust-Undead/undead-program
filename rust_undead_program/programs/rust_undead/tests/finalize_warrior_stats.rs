pub mod common;

use std::str::FromStr;
use {
    common::{
		get_program_test,
		rust_undead_ix_interface,
	},
    solana_program_test::tokio,
    solana_sdk::{
        account::Account, pubkey::Pubkey, rent::Rent, signature::Keypair, signer::Signer, system_program,
    },
};


#[tokio::test]
async fn finalize_warrior_stats_ix_success() {
	let mut program_test = get_program_test();

	// PROGRAMS
	program_test.prefer_bpf(true);

	program_test.add_program(
		"account_compression",
		Pubkey::from_str("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK").unwrap(),
		None,
	);

	program_test.add_program(
		"noop",
		Pubkey::from_str("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV").unwrap(),
		None,
	);

	// KEYPAIR
	let fee_payer_keypair = Keypair::new();
	let warrior_keypair = Keypair::new();

	// PUBKEY
	let fee_payer_pubkey = fee_payer_keypair.pubkey();
	let warrior_pubkey = warrior_keypair.pubkey();
	let owner_pubkey = Pubkey::new_unique();
	let vrf_client_pubkey = Pubkey::new_unique();
	let vrf_program_pubkey = Pubkey::new_unique();

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		fee_payer_pubkey,
		Account {
			lamports: 1_000_000_000_000,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = rust_undead_ix_interface::finalize_warrior_stats_ix_setup(
		&fee_payer_keypair,
		warrior_pubkey,
		owner_pubkey,
		vrf_client_pubkey,
		vrf_program_pubkey,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
