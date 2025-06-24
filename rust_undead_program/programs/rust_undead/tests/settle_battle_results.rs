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
async fn settle_battle_results_ix_success() {
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
	let winner: Pubkey = Pubkey::default();
	let total_questions: u8 = Default::default();
	let player_a_correct: u8 = Default::default();
	let player_b_correct: u8 = Default::default();
	let battle_duration: u32 = Default::default();
	let critical_hits: u8 = Default::default();

	// KEYPAIR
	let fee_payer_keypair = Keypair::new();
	let settlement_authority_keypair = Keypair::new();
	let battle_room_keypair = Keypair::new();
	let warrior_a_keypair = Keypair::new();
	let warrior_b_keypair = Keypair::new();

	// PUBKEY
	let fee_payer_pubkey = fee_payer_keypair.pubkey();
	let settlement_authority_pubkey = settlement_authority_keypair.pubkey();
	let battle_room_pubkey = battle_room_keypair.pubkey();
	let warrior_a_pubkey = warrior_a_keypair.pubkey();
	let warrior_b_pubkey = warrior_b_keypair.pubkey();

	// PDA
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
			lamports: 1_000_000_000_000,
			data: vec![],
			owner: system_program::ID,
			executable: false,
			rent_epoch: 0,
		},
	);

	program_test.add_account(
		settlement_authority_pubkey,
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

	let ix = rust_undead_ix_interface::settle_battle_results_ix_setup(
		&fee_payer_keypair,
		&settlement_authority_keypair,
		battle_room_pubkey,
		warrior_a_pubkey,
		warrior_b_pubkey,
		game_state_pda,
		winner,
		total_questions,
		player_a_correct,
		player_b_correct,
		battle_duration,
		critical_hits,
		recent_blockhash,
	);

	let result = banks_client.process_transaction(ix).await;

	// ASSERTIONS
	assert!(result.is_ok());

}
