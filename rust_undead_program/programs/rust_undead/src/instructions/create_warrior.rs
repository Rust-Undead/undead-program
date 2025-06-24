use crate::*;
use anchor_lang::prelude::*;
use std::str::FromStr;



	#[derive(Accounts)]
	#[instruction(
		name: String,
		dna: u64,
	)]
	pub struct CreateWarrior<'info> {
		pub fee_payer: Signer<'info>,

		pub owner: Signer<'info>,

		#[account(
			init,
			space=197,
			payer=fee_payer,
			seeds = [
				b"warrior",
				owner.key().as_ref(),
				name.as_bytes().as_ref(),
			],
			bump,
		)]
		pub warrior: Account<'info, Warrior>,

		#[account(
			mut,
			seeds = [
				b"game_state",
			],
			bump,
		)]
		pub game_state: Account<'info, GameState>,

		#[account(
			mut,
		)]
		/// CHECK: implement manual checks if needed
		pub vrf_client: UncheckedAccount<'info>,

		/// CHECK: implement manual checks if needed
		pub vrf_program: UncheckedAccount<'info>,

		/// CHECK: implement manual checks if needed
		pub system_program: UncheckedAccount<'info>,
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
pub fn handler(
	ctx: Context<CreateWarrior>,
	name: String,
	dna: u64,
) -> Result<()> {
    // Implement your business logic here...
	
	Ok(())
}
