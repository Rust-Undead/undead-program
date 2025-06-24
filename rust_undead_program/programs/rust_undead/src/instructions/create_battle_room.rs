use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;



	#[derive(Accounts)]
	#[instruction(
		room_id: String,
	)]
	pub struct CreateBattleRoom<'info> {
		pub fee_payer: Signer<'info>,

		pub creator: Signer<'info>,

		#[account(
			mut,
		)]
		pub warrior: Account<'info, Warrior>,

		#[account(
			init,
			space=340,
			payer=fee_payer,
			seeds = [
				b"battle_room",
				room_id.as_bytes().as_ref(),
			],
			bump,
		)]
		pub battle_room: Account<'info, BattleRoom>,

		#[account(
			mut,
			seeds = [
				b"game_state",
			],
			bump,
		)]
		pub game_state: Account<'info, GameState>,

		/// CHECK: implement manual checks if needed
		pub system_program: UncheckedAccount<'info>,
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
pub fn handler(
	ctx: Context<CreateBattleRoom>,
	room_id: String,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
