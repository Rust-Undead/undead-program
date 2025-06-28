use anchor_lang::prelude::*;
use crate::state::*;


pub fn is_warrior_ready(warrior: &UndeadWarrior) -> bool {
    let current_time = Clock::get().unwrap().unix_timestamp;
    current_time >= warrior.cooldown_expires_at
}

// Helper fxns to Calculate warrior achievement based on count
pub fn calculate_warrior_achievement(warrior_count: u32) -> AchievementLevel {
      match warrior_count {
            0 => AchievementLevel::None,
            1..=5 => AchievementLevel::Bronze,
            6..=11 => AchievementLevel::Silver,
            12..=20 => AchievementLevel::Gold,
            21..=50 => AchievementLevel::Platinum,
            _ => AchievementLevel::Diamond,
        }
    }
    
    // Calculate overall achievement based on total points
    pub fn calculate_overall_achievement(total_points: u64) -> AchievementLevel {
        match total_points {
            0..=99 => AchievementLevel::None,
            100..=499 => AchievementLevel::Bronze,
            500..=1499 => AchievementLevel::Silver,
            1500..=4999 => AchievementLevel::Gold,
            5000..=14999 => AchievementLevel::Platinum,
            _ => AchievementLevel::Diamond,
        }
    }

    pub fn calculate_winner_achievement(wins: u32) -> AchievementLevel {
    match wins {
        0 => AchievementLevel::None,
        1..=2 => AchievementLevel::Bronze,     // 1-2 wins
        3..=9 => AchievementLevel::Silver,     // 3-9 wins  
        10..=24 => AchievementLevel::Gold,     // 10-24 wins
        25..=49 => AchievementLevel::Platinum, // 25-49 wins
        50.. => AchievementLevel::Diamond,     // 50+ wins
    }
}

pub fn calculate_battle_achievement(battles: u32) -> AchievementLevel {
    match battles {
        0..=4 => AchievementLevel::None,       // 0-4 battles
        5..=14 => AchievementLevel::Bronze,    // 5-14 battles
        15..=39 => AchievementLevel::Silver,   // 15-39 battles
        40..=99 => AchievementLevel::Gold,     // 40-99 battles
        100..=199 => AchievementLevel::Platinum, // 100-199 battles
        200.. => AchievementLevel::Diamond,    // 200+ battles
    }
}
