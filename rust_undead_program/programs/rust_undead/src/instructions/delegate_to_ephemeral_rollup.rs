use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;



	#[derive(Accounts)]
	#[instruction(
		er_session_id: u64,
	)]
	pub struct DelegateToEphemeralRollup<'info> {
		pub fee_payer: Signer<'info>,

		pub authority: Signer<'info>,

		#[account(
			mut,
		)]
		pub battle_room: Account<'info, BattleRoom>,

		#[account(
			mut,
		)]
		pub warrior_a: Account<'info, Warrior>,

		#[account(
			mut,
		)]
		pub warrior_b: Account<'info, Warrior>,
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
pub fn handler(
	ctx: Context<DelegateToEphemeralRollup>,
	er_session_id: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
