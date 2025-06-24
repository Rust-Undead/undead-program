use anchor_lang::*;


#[account]
#[derive(InitSpace)]
// Leaderboard account
pub struct Leaderboard { 
pub top_players: [Pubkey; 20],
pub top_scores: [u64; 20], 
pub last_updated: i64,
}

impl Leaderboard {
  pub fn update_player_score(&mut self, player: Pubkey, new_score: u64) -> Result<()> {
        // Check if player is already in leaderboard
        let mut existing_position: Option<usize> = None;
        for i in 0..20 {
            if self.top_players[i] == player {
                existing_position = Some(i);
                break;
            }
        }
        match existing_position {
            Some(pos) => {
                self.top_scores[pos] = new_score;
            }
            None => {
                // Player not in leaderboard - try to add them
                self.add_new_player(player, new_score)?;
            }
        }

        // Sort leaderboard by score (descending)
        self.sort_leaderboard();
        self.last_updated = Clock::get()?.unix_timestamp;
        
        Ok(())
    }

}