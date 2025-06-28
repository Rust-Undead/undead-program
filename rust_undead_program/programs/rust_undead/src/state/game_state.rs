use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
	pub admin: Pubkey,
	pub total_warriors: u64,
    pub cooldown_time: u64,
	pub total_battles: u64,
	pub is_paused: bool,
	pub created_at: i64,
    pub bump: u8, // Bump seed for PDA
}

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    pub owner: Pubkey,
    pub warriors_created: u32, 
	pub story_level: u32,     
    pub total_battles_won: u32,
	pub total_battles_lost: u32,    
    pub total_battles_fought: u32,    
    pub join_date: i64,
    pub total_points:u64,  
    pub bump: u8,                     
}


#[account]
#[derive(InitSpace)]
pub struct UserAchievements {
    pub owner: Pubkey,
    pub overall_achievements: AchievementLevel, // Based on total points and ranking in leaderboard
    pub warrior_achivement: AchievementLevel,   // Based on warriors created
    pub winner_achievement: AchievementLevel,   // Based on battles won
    pub battle_achievement: AchievementLevel,   // Based on total battles fought
    pub first_warrior_date: i64,              
    pub first_victory_date: i64,   
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum AchievementLevel {
    None,
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
}


impl Space for AchievementLevel {
	const INIT_SPACE: usize = 1;
 }
 