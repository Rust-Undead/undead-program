use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;



	#[derive(Accounts)]
	pub struct JoinBattleRoom<'info> {
		pub fee_payer: Signer<'info>,

		pub joiner: Signer<'info>,

		#[account(
			mut,
		)]
		pub warrior: Account<'info, Warrior>,

		#[account(
			mut,
		)]
		pub battle_room: Account<'info, BattleRoom>,
	}

/// Join an existing battle room
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` joiner: [AccountInfo] The player joining the battle
/// 2. `[writable]` warrior: [Warrior] The joiner's warrior
/// 3. `[writable]` battle_room: [BattleRoom] The battle room account
pub fn handler(
	ctx: Context<JoinBattleRoom>,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
