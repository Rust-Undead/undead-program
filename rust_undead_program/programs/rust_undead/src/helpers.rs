use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
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
    pub fn calculate_overall_achievement(total_points: u32) -> AchievementLevel {
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

pub fn calculate_deterministic_damage_with_keys(
    attacker_warrior: &UndeadWarrior,
    defender_warrior: &UndeadWarrior,
    attacker_key: Pubkey,
    defender_key: Pubkey,
    current_q: usize,
    room_id: [u8; 32],
    client_seed: u8,
) -> Result<u16> {
    // ✅ DETERMINISTIC DAMAGE CALCULATION
    // Create unique seed for this damage event
    let mut seed_data = Vec::new();
    seed_data.extend_from_slice(&room_id);
    seed_data.push(current_q as u8);
    seed_data.extend_from_slice(&attacker_key.to_bytes());
    seed_data.extend_from_slice(&defender_key.to_bytes());
    seed_data.push(client_seed);  // Add some variation
    
    // Hash the seed data to get pseudo-random value
    let hash_result = hash(&seed_data);
    let pseudo_random = u16::from_le_bytes([hash_result.to_bytes()[0], hash_result.to_bytes()[1]]);
    
    // Get damage range based on question phase (escalating difficulty)
    let (min_damage, max_damage) = match current_q {
        0..=2 => (2, 10),    // Questions 1-3: Learning Phase
        3..=6 => (6, 15),    // Questions 4-7: Pressure Phase
        7..=9 => (10, 20),   // Questions 8-10: Deadly Phase
        _ => (1, 1),         // Fallback
    };
    
    // Scale pseudo-random to damage range
    let damage_range = max_damage - min_damage + 1;
    let base_damage = min_damage + ((pseudo_random % damage_range as u16) as u8);
    
    // Apply warrior stat modifiers  
    let attack_bonus = attacker_warrior.base_attack as i32; 
    let defense_reduction = defender_warrior.base_defense as i32; 
    //knowledge of the warrior 
    let knowledge_impact = defender_warrior.base_knowledge as i32; 
    let stat_modifier = (attack_bonus - (defense_reduction + knowledge_impact )) / 10;
    
    // Calculate final damage with minimum of 1
    let final_damage = (base_damage as i32 + stat_modifier).max(1) as u16;
    
    // Log damage calculation details
    msg!("🎲 Deterministic Damage Calculation:");
    msg!("   Question Phase: {} (Q{})", 
        match current_q {
            0..=2 => "Learning",
            3..=6 => "Pressure",
            7..=9 => "Deadly",
            _ => "Unknown"
        },
        current_q + 1
    );
    msg!("   Base Damage Range: {}-{}", min_damage, max_damage);
    msg!("   Calculated Base Damage: {}", base_damage);
    msg!("   Attacker {} ATK: {}", attacker_warrior.name, attacker_warrior.base_attack);
    msg!("   Defender {} DEF: {}", defender_warrior.name, defender_warrior.base_defense);
    msg!("   Stat Modifier: {}", stat_modifier);
    msg!("   Final Damage: {}", final_damage);
    
    Ok(final_damage)
}