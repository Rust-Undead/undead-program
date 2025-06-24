
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use std::str::FromStr;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9aVsYoGKsTMBTCEZ2K2UCfUJRV6X7PCCrz8txENGuJ3d");

#[program]
pub mod rust_undead {
    use super::*;

/// Initialize the global game state
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` authority: [AccountInfo] The game authority
/// 2. `[writable]` game_state: [GameState] The global game state account
/// 3. `[]` system_program: [AccountInfo] The system program
///
/// Data:
/// - vrf_oracle: [Pubkey] Magic Block VRF oracle pubkey
/// - er_bridge_authority: [Pubkey] Ephemeral rollup bridge authority
	pub fn initialize_game(ctx: Context<InitializeGame>, vrf_oracle: Pubkey, er_bridge_authority: Pubkey) -> Result<()> {
		initialize_game::handler(ctx, vrf_oracle, er_bridge_authority)
	}

/// Create a new warrior with DNA and request VRF for stats
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` owner: [AccountInfo] The owner of the warrior
/// 2. `[writable]` warrior: [Warrior] The warrior account
/// 3. `[writable]` game_state: [GameState] The global game state account
/// 4. `[writable]` vrf_client: [AccountInfo] The Magic Block VRF client account
/// 5. `[]` vrf_program: [AccountInfo] The Magic Block VRF program
/// 6. `[]` system_program: [AccountInfo] The system program
///
/// Data:
/// - name: [String] The name of the warrior
/// - dna: [u64] DNA for visual generation
	pub fn create_warrior(ctx: Context<CreateWarrior>, name: String, dna: u64) -> Result<()> {
		create_warrior::handler(ctx, name, dna)
	}

/// Finalize warrior creation with VRF randomness result
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` warrior: [Warrior] The warrior account
/// 2. `[]` owner: [AccountInfo] The warrior owner (for verification)
/// 3. `[]` vrf_client: [AccountInfo] The Magic Block VRF client account
/// 4. `[]` vrf_program: [AccountInfo] The Magic Block VRF program
	pub fn finalize_warrior_stats(ctx: Context<FinalizeWarriorStats>) -> Result<()> {
		finalize_warrior_stats::handler(ctx)
	}

/// Create a new battle room
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` creator: [AccountInfo] The creator of the battle room
/// 2. `[writable]` warrior: [Warrior] The creator's warrior
/// 3. `[writable]` battle_room: [BattleRoom] The battle room account
/// 4. `[writable]` game_state: [GameState] The global game state account
/// 5. `[]` system_program: [AccountInfo] The system program
///
/// Data:
/// - room_id: [String] Unique identifier for the battle room
	pub fn create_battle_room(ctx: Context<CreateBattleRoom>, room_id: String) -> Result<()> {
		create_battle_room::handler(ctx, room_id)
	}

/// Join an existing battle room
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` joiner: [AccountInfo] The player joining the battle
/// 2. `[writable]` warrior: [Warrior] The joiner's warrior
/// 3. `[writable]` battle_room: [BattleRoom] The battle room account
	pub fn join_battle_room(ctx: Context<JoinBattleRoom>) -> Result<()> {
		join_battle_room::handler(ctx)
	}

/// Use VRF to select 5 concepts for the battle
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` authority: [AccountInfo] Either player can trigger concept selection
/// 2. `[writable]` battle_room: [BattleRoom] The battle room account
/// 3. `[writable]` vrf_client: [AccountInfo] The Magic Block VRF client account
/// 4. `[]` vrf_program: [AccountInfo] The Magic Block VRF program
	pub fn select_battle_concepts(ctx: Context<SelectBattleConcepts>) -> Result<()> {
		select_battle_concepts::handler(ctx)
	}

/// Finalize concept selection with VRF result
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` battle_room: [BattleRoom] The battle room account
/// 2. `[]` vrf_client: [AccountInfo] The Magic Block VRF client account
/// 3. `[]` vrf_program: [AccountInfo] The Magic Block VRF program
	pub fn finalize_concept_selection(ctx: Context<FinalizeConceptSelection>) -> Result<()> {
		finalize_concept_selection::handler(ctx)
	}

/// Mark a player as ready for battle after studying concepts
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` player: [AccountInfo] The player marking themselves as ready
/// 2. `[writable]` battle_room: [BattleRoom] The battle room account
	pub fn mark_ready_for_battle(ctx: Context<MarkReadyForBattle>) -> Result<()> {
		mark_ready_for_battle::handler(ctx)
	}

/// Delegate warriors to ephemeral rollup for real-time battle
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` authority: [AccountInfo] Either player can trigger delegation
/// 2. `[writable]` battle_room: [BattleRoom] The battle room account
/// 3. `[writable]` warrior_a: [Warrior] First warrior in the battle
/// 4. `[writable]` warrior_b: [Warrior] Second warrior in the battle
///
/// Data:
/// - er_session_id: [u64] Ephemeral rollup session identifier
	pub fn delegate_to_ephemeral_rollup(ctx: Context<DelegateToEphemeralRollup>, er_session_id: u64) -> Result<()> {
		delegate_to_ephemeral_rollup::handler(ctx, er_session_id)
	}

/// Settle battle results from ephemeral rollup back to mainnet
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` settlement_authority: [AccountInfo] Authorized settlement bridge or admin
/// 2. `[writable]` battle_room: [BattleRoom] The battle room account
/// 3. `[writable]` warrior_a: [Warrior] First warrior in the battle
/// 4. `[writable]` warrior_b: [Warrior] Second warrior in the battle
/// 5. `[writable]` game_state: [GameState] The global game state account
///
/// Data:
/// - winner: [Pubkey] The winner of the battle
/// - total_questions: [u8] Total questions in the battle
/// - player_a_correct: [u8] Correct answers by player A
/// - player_b_correct: [u8] Correct answers by player B
/// - battle_duration: [u32] Battle duration in seconds
/// - critical_hits: [u8] Total critical hits
	pub fn settle_battle_results(ctx: Context<SettleBattleResults>, winner: Pubkey, total_questions: u8, player_a_correct: u8, player_b_correct: u8, battle_duration: u32, critical_hits: u8) -> Result<()> {
		settle_battle_results::handler(ctx, winner, total_questions, player_a_correct, player_b_correct, battle_duration, critical_hits)
	}

/// Update game configuration (admin only)
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` authority: [AccountInfo] The game authority
/// 2. `[writable]` game_state: [GameState] The global game state account
///
/// Data:
/// - new_warrior_creation_fee: [u64] New warrior creation fee (optional)
/// - new_battle_entry_fee: [u64] New battle entry fee (optional)
/// - new_vrf_oracle: [Pubkey] New VRF oracle (optional)
	pub fn update_game_config(ctx: Context<UpdateGameConfig>, new_warrior_creation_fee: u64, new_battle_entry_fee: u64, new_vrf_oracle: Pubkey) -> Result<()> {
		update_game_config::handler(ctx, new_warrior_creation_fee, new_battle_entry_fee, new_vrf_oracle)
	}



}
