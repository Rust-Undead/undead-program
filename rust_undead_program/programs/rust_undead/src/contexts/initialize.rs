use anchor_lang::prelude::*;
use crate::state::{Config,Leaderboard};
use crate::constants::{ANCHOR_DISCRIMINATOR, CONFIG, LEADERBOARD};

	#[derive(Accounts)]
	pub struct Initialize<'info> {
		#[account(mut)]
		pub authority: Signer<'info>,

		#[account(
			init,
			payer = authority,
			space = ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
			seeds = [CONFIG, authority.key().as_ref()],
			bump,
		)]
		pub config: Account<'info, Config>,

		#[account(
			init,
			payer = authority,
			space = ANCHOR_DISCRIMINATOR + Leaderboard::INIT_SPACE,
			seeds = [LEADERBOARD, authority.key().as_ref()],
			bump,
		)]
		pub leaderboard: Account<'info, Leaderboard>,
		pub system_program : Program<'info, System>
	}

	impl <'info> Initialize <'info> {
		pub fn initialize (
		&mut self,
		cooldown_time: u64,
		bumps: &InitializeBumps
		) -> Result<()> {
		let clock = Clock::get()?;

		self.config.set_inner(
			Config { 
				admin: self.authority.key(), 
				total_warriors: 0, 
				cooldown_time, 
				total_battles: 0, 
				is_paused: false, 
				created_at: clock.unix_timestamp,
				bump: bumps.config,
			 });

			 self.leaderboard.set_inner(
				Leaderboard{
					top_players: [Pubkey::default(); 20],
					top_scores: [0u64; 20], 
					last_updated: clock.unix_timestamp,
					bump: bumps.leaderboard,
				}
			 );
			Ok(())
		}
	}
	

