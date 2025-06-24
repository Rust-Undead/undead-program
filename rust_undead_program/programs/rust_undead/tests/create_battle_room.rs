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
async fn create_battle_room_ix_success() {
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

	// DATA
	let room_id: String = Default::default();

	// KEYPAIR
	let fee_payer_keypair = Keypair::new();
	let creator_keypair = Keypair::new();
	let warrior_keypair = Keypair::new();

	// PUBKEY
	let fee_payer_pubkey = fee_payer_keypair.pubkey();
	let creator_pubkey = creator_keypair.pubkey();
	let warrior_pubkey = warrior_keypair.pubkey();
	let system_program_pubkey = Pubkey::new_unique();

	// PDA
	let (battle_room_pda, _battle_room_pda_bump) = Pubkey::find_program_address(
		&[
			b"battle_room",
			room_id.as_bytes().as_ref(),
		],
		&rust_undead::ID,
	);

	let (game_state_pda, _game_state_pda_bump) = Pubkey::find_program_address(
		&[
			b"game_state",
		],
		&rust_undead::ID,
	);

	// ACCOUNT PROGRAM TEST SETUP
	program_test.add_account(
		fee_payer_pubkey,
		Account {
			lamports: 0,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	program_test.add_account(
		creator_pubkey,
		Account {
			lamports: 0,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	// INSTRUCTIONS
	let (mut banks_client, _, recent_blockhash) = program_test.start().await;

	let ix = rust_undead_ix_interface::create_battle_room_ix_setup(
		&fee_payer_keypair,
		&creator_keypair,
		warrior_pubkey,
		battle_room_pda,
		game_state_pda,
		system_program_pubkey,
		&room_id,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
