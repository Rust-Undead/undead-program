use crate::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use std::str::FromStr;
use crate::state::{Config,  Leaderboard};
use crate::constants::ANCHOR_DISCRIMINATOR;


	#[derive(Accounts)]
	pub struct Initialize<'info> {
		#[account(mut)]
		pub authority: Signer<'info>,

		#[account(
			init,
			payer = authority,
			space = ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
			seeds = [b"config", authority.key().as_ref()],
			bump,
		)]
		pub config: Account<'info, Config>,

		#[account(
			init,
			payer = authority,
			space = ANCHOR_DISCRIMINATOR + Leaderboard::INIT_SPACE,
			seeds = [b"leaderboard", authority.key().as_ref()],
			bump,
		)]
		pub leaderboard: Account<'info, Leaderboard>,
		pub system_program : Program<'info, System>
	}

	impl <'info> Initialize <'info> {
		pub fn initialize (
		&mut self,
		) -> Result<()> {
		let clock = Clock::get()?;

		self.config.set_inner(
			Config { 
				admin: authority.key(), 
				total_warriors: 0, 
				total_battles: 0, 
				is_paused: 0, 
				created_at: clock::UnixTimestamp(),
			 });

			 self.leaderboard.set_inner(
				Leaderboard{
					top_players: [Pubkey; 20],
					top_scores: [u64; 20], 
					last_updated: i64
				}
			 )

			 
			Ok(())
		}
	}
	

