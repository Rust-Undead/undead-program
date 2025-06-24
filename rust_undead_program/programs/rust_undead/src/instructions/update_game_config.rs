use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;



	#[derive(Accounts)]
	#[instruction(
		new_warrior_creation_fee: u64,
		new_battle_entry_fee: u64,
		new_vrf_oracle: Pubkey,
	)]
	pub struct UpdateGameConfig<'info> {
		pub fee_payer: Signer<'info>,

		pub authority: Signer<'info>,

		#[account(
			mut,
			seeds = [
				b"game_state",
			],
			bump,
		)]
		pub game_state: Account<'info, GameState>,
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
pub fn handler(
	ctx: Context<UpdateGameConfig>,
	new_warrior_creation_fee: u64,
	new_battle_entry_fee: u64,
	new_vrf_oracle: Pubkey,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
