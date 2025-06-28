use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Leaderboard { 
    pub top_players: [Pubkey; 20],
    pub top_scores: [u64; 20], 
    pub last_updated: i64,
    pub bump: u8,
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
                // Player exists - update their score
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

    fn add_new_player(&mut self, player: Pubkey, score: u64) -> Result<()> {
        // Find the lowest score position or empty slot
        let mut insert_position: Option<usize> = None;
        

        for i in 0..20 {
            if self.top_players[i] == Pubkey::default() {
                insert_position = Some(i);
                break;
            }
        }
        
        // If no empty slots, check if score qualifies for leaderboard
        if insert_position.is_none() {
            // Find the lowest score
            let mut lowest_score_pos = 19; // Start with last position
            for i in 0..20 {
                if self.top_scores[i] < self.top_scores[lowest_score_pos] {
                    lowest_score_pos = i;
                }
            }
            
            // Only add if new score is higher than lowest score
            if score > self.top_scores[lowest_score_pos] {
                insert_position = Some(lowest_score_pos);
            }
        }
        
        // Insert the player if position found
        if let Some(pos) = insert_position {
            self.top_players[pos] = player;
            self.top_scores[pos] = score;
            msg!("Player {} added to leaderboard at position {} with score {}", player, pos + 1, score);
        } else {
            msg!("Player {} score {} not high enough for leaderboard", player, score);
        }
        
        Ok(())
    }

    fn sort_leaderboard(&mut self) {
       
        let mut paired: Vec<(Pubkey, u64)> = Vec::new();
        
        for i in 0..20 {
            paired.push((self.top_players[i], self.top_scores[i]));
        }
        
        paired.sort_by(|a, b| {
            match (a.0 == Pubkey::default(), b.0 == Pubkey::default()) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                (true, true) => std::cmp::Ordering::Equal,     
                (false, false) => b.1.cmp(&a.1),            
            }
        });
        
        // Update the arrays with sorted data
        for i in 0..20 {
            self.top_players[i] = paired[i].0;
            self.top_scores[i] = paired[i].1;
        }
    }

    pub fn get_player_rank(&self, player: Pubkey) -> Option<usize> {
        for i in 0..20 {
            if self.top_players[i] == player {
                return Some(i + 1); // Return 1-based rank
            }
        }
        None
    }

    pub fn is_top_10(&self, player: Pubkey) -> bool {
        for i in 0..10 {
            if self.top_players[i] == player {
                return true;
            }
        }
        false
    }
    pub fn get_top_10(&self) -> Vec<(Pubkey, u64)> {
        let mut top_10 = Vec::new();
        for i in 0..10 {
            if self.top_players[i] != Pubkey::default() {
                top_10.push((self.top_players[i], self.top_scores[i]));
            }
        }
        top_10
    }
    pub fn initialize(&mut self) -> Result<()> {
        self.top_players = [Pubkey::default(); 20];
        self.top_scores = [0u64; 20];
        self.last_updated = Clock::get()?.unix_timestamp;
        msg!("Leaderboard initialized");
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        self.top_players = [Pubkey::default(); 20];
        self.top_scores = [0u64; 20];
        self.last_updated = Clock::get()?.unix_timestamp;
        msg!("Leaderboard reset");
        Ok(())
    }
}