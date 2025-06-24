use {
	rust_undead::{
			entry,
			ID as PROGRAM_ID,
	},
	solana_sdk::{
		entrypoint::{ProcessInstruction, ProgramResult},
		pubkey::Pubkey,
	},
	anchor_lang::prelude::AccountInfo,
	solana_program_test::*,
};

// Type alias for the entry function pointer used to convert the entry function into a ProcessInstruction function pointer.
pub type ProgramEntry = for<'info> fn(
	program_id: &Pubkey,
	accounts: &'info [AccountInfo<'info>],
	instruction_data: &[u8],
) -> ProgramResult;

// Macro to convert the entry function into a ProcessInstruction function pointer.
#[macro_export]
macro_rules! convert_entry {
	($entry:expr) => {
		// Use unsafe block to perform memory transmutation.
		unsafe { core::mem::transmute::<ProgramEntry, ProcessInstruction>($entry) }
	};
}

pub fn get_program_test() -> ProgramTest {
	let program_test = ProgramTest::new(
		"rust_undead",
		PROGRAM_ID,
		processor!(convert_entry!(entry)),
	);
	program_test
}
	
pub mod rust_undead_ix_interface {

	use {
		solana_sdk::{
			hash::Hash,
			signature::{Keypair, Signer},
			instruction::Instruction,
			pubkey::Pubkey,
			transaction::Transaction,
		},
		rust_undead::{
			ID as PROGRAM_ID,
			accounts as rust_undead_accounts,
			instruction as rust_undead_instruction,
		},
		anchor_lang::{
			prelude::*,
			InstructionData,
		}
	};

	pub fn initialize_game_ix_setup(
		fee_payer: &Keypair,
		authority: &Keypair,
		game_state: Pubkey,
		system_program: Pubkey,
		vrf_oracle: Pubkey,
		er_bridge_authority: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::InitializeGame {
			fee_payer: fee_payer.pubkey(),
			authority: authority.pubkey(),
			game_state: game_state,
			system_program: system_program,
		};

		let data = 	rust_undead_instruction::InitializeGame {
				vrf_oracle,
				er_bridge_authority,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn create_warrior_ix_setup(
		fee_payer: &Keypair,
		owner: &Keypair,
		warrior: Pubkey,
		game_state: Pubkey,
		vrf_client: Pubkey,
		vrf_program: Pubkey,
		system_program: Pubkey,
		name: &String,
		dna: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::CreateWarrior {
			fee_payer: fee_payer.pubkey(),
			owner: owner.pubkey(),
			warrior: warrior,
			game_state: game_state,
			vrf_client: vrf_client,
			vrf_program: vrf_program,
			system_program: system_program,
		};

		let data = 	rust_undead_instruction::CreateWarrior {
				name: name.clone(),
				dna,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&owner,
		], recent_blockhash);

		return transaction;
	}

	pub fn finalize_warrior_stats_ix_setup(
		fee_payer: &Keypair,
		warrior: Pubkey,
		owner: Pubkey,
		vrf_client: Pubkey,
		vrf_program: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::FinalizeWarriorStats {
			fee_payer: fee_payer.pubkey(),
			warrior: warrior,
			owner: owner,
			vrf_client: vrf_client,
			vrf_program: vrf_program,
		};

		let data = rust_undead_instruction::FinalizeWarriorStats;
		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
		], recent_blockhash);

		return transaction;
	}

	pub fn create_battle_room_ix_setup(
		fee_payer: &Keypair,
		creator: &Keypair,
		warrior: Pubkey,
		battle_room: Pubkey,
		game_state: Pubkey,
		system_program: Pubkey,
		room_id: &String,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::CreateBattleRoom {
			fee_payer: fee_payer.pubkey(),
			creator: creator.pubkey(),
			warrior: warrior,
			battle_room: battle_room,
			game_state: game_state,
			system_program: system_program,
		};

		let data = 	rust_undead_instruction::CreateBattleRoom {
				room_id: room_id.clone(),
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&creator,
		], recent_blockhash);

		return transaction;
	}

	pub fn join_battle_room_ix_setup(
		fee_payer: &Keypair,
		joiner: &Keypair,
		warrior: Pubkey,
		battle_room: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::JoinBattleRoom {
			fee_payer: fee_payer.pubkey(),
			joiner: joiner.pubkey(),
			warrior: warrior,
			battle_room: battle_room,
		};

		let data = rust_undead_instruction::JoinBattleRoom;
		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&joiner,
		], recent_blockhash);

		return transaction;
	}

	pub fn select_battle_concepts_ix_setup(
		fee_payer: &Keypair,
		authority: &Keypair,
		battle_room: Pubkey,
		vrf_client: Pubkey,
		vrf_program: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::SelectBattleConcepts {
			fee_payer: fee_payer.pubkey(),
			authority: authority.pubkey(),
			battle_room: battle_room,
			vrf_client: vrf_client,
			vrf_program: vrf_program,
		};

		let data = rust_undead_instruction::SelectBattleConcepts;
		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn finalize_concept_selection_ix_setup(
		fee_payer: &Keypair,
		battle_room: Pubkey,
		vrf_client: Pubkey,
		vrf_program: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::FinalizeConceptSelection {
			fee_payer: fee_payer.pubkey(),
			battle_room: battle_room,
			vrf_client: vrf_client,
			vrf_program: vrf_program,
		};

		let data = rust_undead_instruction::FinalizeConceptSelection;
		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
		], recent_blockhash);

		return transaction;
	}

	pub fn mark_ready_for_battle_ix_setup(
		fee_payer: &Keypair,
		player: &Keypair,
		battle_room: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::MarkReadyForBattle {
			fee_payer: fee_payer.pubkey(),
			player: player.pubkey(),
			battle_room: battle_room,
		};

		let data = rust_undead_instruction::MarkReadyForBattle;
		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&player,
		], recent_blockhash);

		return transaction;
	}

	pub fn delegate_to_ephemeral_rollup_ix_setup(
		fee_payer: &Keypair,
		authority: &Keypair,
		battle_room: Pubkey,
		warrior_a: Pubkey,
		warrior_b: Pubkey,
		er_session_id: u64,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::DelegateToEphemeralRollup {
			fee_payer: fee_payer.pubkey(),
			authority: authority.pubkey(),
			battle_room: battle_room,
			warrior_a: warrior_a,
			warrior_b: warrior_b,
		};

		let data = 	rust_undead_instruction::DelegateToEphemeralRollup {
				er_session_id,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn settle_battle_results_ix_setup(
		fee_payer: &Keypair,
		settlement_authority: &Keypair,
		battle_room: Pubkey,
		warrior_a: Pubkey,
		warrior_b: Pubkey,
		game_state: Pubkey,
		winner: Pubkey,
		total_questions: u8,
		player_a_correct: u8,
		player_b_correct: u8,
		battle_duration: u32,
		critical_hits: u8,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::SettleBattleResults {
			fee_payer: fee_payer.pubkey(),
			settlement_authority: settlement_authority.pubkey(),
			battle_room: battle_room,
			warrior_a: warrior_a,
			warrior_b: warrior_b,
			game_state: game_state,
		};

		let data = 	rust_undead_instruction::SettleBattleResults {
				winner,
				total_questions,
				player_a_correct,
				player_b_correct,
				battle_duration,
				critical_hits,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&settlement_authority,
		], recent_blockhash);

		return transaction;
	}

	pub fn update_game_config_ix_setup(
		fee_payer: &Keypair,
		authority: &Keypair,
		game_state: Pubkey,
		new_warrior_creation_fee: u64,
		new_battle_entry_fee: u64,
		new_vrf_oracle: Pubkey,
		recent_blockhash: Hash,
	) -> Transaction {
		let accounts = rust_undead_accounts::UpdateGameConfig {
			fee_payer: fee_payer.pubkey(),
			authority: authority.pubkey(),
			game_state: game_state,
		};

		let data = 	rust_undead_instruction::UpdateGameConfig {
				new_warrior_creation_fee,
				new_battle_entry_fee,
				new_vrf_oracle,
		};		let instruction = Instruction::new_with_bytes(PROGRAM_ID, &data.data(), accounts.to_account_metas(None));
		let mut transaction = Transaction::new_with_payer(
			&[instruction], 
			Some(&fee_payer.pubkey()),
		);

		transaction.sign(&[
			&fee_payer,
			&authority,
		], recent_blockhash);

		return transaction;
	}

}
