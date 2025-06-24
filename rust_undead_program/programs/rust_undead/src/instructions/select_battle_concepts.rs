use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;



	#[derive(Accounts)]
	pub struct SelectBattleConcepts<'info> {
		pub fee_payer: Signer<'info>,

		pub authority: Signer<'info>,

		#[account(
			mut,
		)]
		pub battle_room: Account<'info, BattleRoom>,

		#[account(
			mut,
		)]
		/// CHECK: implement manual checks if needed
		pub vrf_client: UncheckedAccount<'info>,

		/// CHECK: implement manual checks if needed
		pub vrf_program: UncheckedAccount<'info>,
	}

/// Use VRF to select 5 concepts for the battle
///
/// Accounts:
/// 0. `[signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` authority: [AccountInfo] Either player can trigger concept selection
/// 2. `[writable]` battle_room: [BattleRoom] The battle room account
/// 3. `[writable]` vrf_client: [AccountInfo] The Magic Block VRF client account
/// 4. `[]` vrf_program: [AccountInfo] The Magic Block VRF program
pub fn handler(
	ctx: Context<SelectBattleConcepts>,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
