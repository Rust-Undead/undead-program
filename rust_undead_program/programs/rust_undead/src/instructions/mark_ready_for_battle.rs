use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;



	#[derive(Accounts)]
	pub struct MarkReadyForBattle<'info> {
		pub fee_payer: Signer<'info>,

		pub player: Signer<'info>,

		#[account(
			mut,
		)]
		pub battle_room: Account<'info, BattleRoom>,
	}

/// Mark a player as ready for battle after studying concepts
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` player: [AccountInfo] The player marking themselves as ready
/// 2. `[writable]` battle_room: [BattleRoom] The battle room account
pub fn handler(
	ctx: Context<MarkReadyForBattle>,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
