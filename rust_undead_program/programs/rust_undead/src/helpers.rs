use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;
use crate::state::*;
use crate::constants::*;

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

pub fn get_class_folder_hash(class: WarriorClass) -> &'static str {
    match class {
        WarriorClass::Guardian => GUARDIAN_FOLDER_HASH,
        WarriorClass::Validator => VALIDATOR_FOLDER_HASH,
        WarriorClass::Oracle => ORACLE_FOLDER_HASH,
        WarriorClass::Daemon => DAEMON_FOLDER_HASH,
    }
}

// Generate image selection and URL from VRF randomness
// pub fn generate_warrior_image(
//     randomness_slice: &[u8; 32],
//     class: WarriorClass
// ) -> Result<(ImageRarity, u8, String)> {
    
//     // Step 1: Determine rarity using VRF (65% common, 25% uncommon, 10% rare) range 1-100
//     let rarity_roll = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness_slice, 1, 101);
    
//     let (rarity, rarity_prefix, max_count) = match rarity_roll {
//         1..=65 => (ImageRarity::Common, "c", COMMON_COUNT),      // 65% chance
//         66..=90 => (ImageRarity::Uncommon, "u", UNCOMMON_COUNT), // 25% chance
//         91..=100 => (ImageRarity::Rare, "r", RARE_COUNT),        // 10% chance
//         _ => (ImageRarity::Common, "c", COMMON_COUNT),
//     };
    
//     // Step 2: Select random image within rarity (1-based numbering: c1, c2, etc.)
//     let image_number = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness_slice, 1, max_count + 1);
    
//     // Step 3: Construct full IPFS URL
//     let folder_hash = get_class_folder_hash(class);
//     let image_url = format!(
//         "{}/{}/{}{}.png",
//         IPFS_GATEWAY,
//         folder_hash,
//         rarity_prefix,
//         image_number
//     );
    
//     msg!(
//         "🎨 Selected {} {} image: {}{}.png → {}",
//         class.to_string(),
//         rarity.to_string(),
//         rarity_prefix,
//         image_number,
//         image_url
//     );
    
//     Ok((rarity, image_number, image_url))
// }


pub fn temp_img_rand(
    player_key: Pubkey,
    dna: [u8; 8], 
    client_seed: u8,
    class: WarriorClass
) -> Result<(ImageRarity, u8, String)> {

     msg!("Generating random Image Stats for warrior: {} (class: {:?}) for", player_key, class);

    //first set seeds 
    let mut seed_data = Vec::new();
    //1. extend the vec from slice of roomid 
    seed_data.extend_from_slice(&dna);
    //2. push the client seed
    seed_data.push(client_seed);
    // 3. Extend the player key
    seed_data.extend_from_slice(&player_key.to_bytes());

    //4. Hash the seed to get a pseudo-random value
    let hash_result = hash(&seed_data);
    //  this gives us a u16 
    let rarity_random = u16::from_le_bytes([hash_result.to_bytes()[0], hash_result.to_bytes()[1]]);
    // determine range (65% common, 25% uncommon, 10% rare) range 1-100

    let image_index_random = u16::from_le_bytes([hash_result.to_bytes()[2], hash_result.to_bytes()[3]]);
    // since u16 range is 0 - 65535 we can have the modulo with 100 where it is either 1 - 100 and not more 
    let rarity_roll = (rarity_random%100) + 1; // range 1 - 100
    
    let (rarity, rarity_prefix, max_count) = match rarity_roll {
        1..=65 => (ImageRarity::Common, "c", COMMON_COUNT),
        66..=90 => (ImageRarity::Uncommon, "u", UNCOMMON_COUNT),
        91..=100 => (ImageRarity::Rare, "r", RARE_COUNT),
        _=> (ImageRarity::Common, "c", COMMON_COUNT),
    };

    let image_number = ((image_index_random % max_count as u16) + 1) as u8;

    // Step 3: Construct full IPFS URL
    let folder_hash = get_class_folder_hash(class);
    let image_url = format!(
        "{}/{}/{}{}.png",
        IPFS_GATEWAY,
        folder_hash,
        rarity_prefix,
        image_number
    );
    
    msg!(
        "🎨 Selected {} {} image: {}{}.png → {}",
        class.to_string(),
        rarity.to_string(),
        rarity_prefix,
        image_number,
        image_url
    );
    
    Ok((rarity, image_number, image_url))



}


// stats random 
pub fn temp_stats_rand(
    player_key: Pubkey,
    dna: [u8; 8], 
    client_seed: u8,
    class: WarriorClass,
    
) -> Result<(u16, u16, u16)> {

msg!("Generating random combat stats for warrior: {} (class: {:?}) for ", player_key, class);

let mut seed_data = Vec::new();
    //1. extend the vec from slice of roomid 
    seed_data.extend_from_slice(&dna);
    //2. push the client seed
    seed_data.push(client_seed);
    // 3. Extend the player key
    seed_data.extend_from_slice(&player_key.to_bytes());
    let hash_results = hash(&seed_data);

    let attack_rand = u16::from_le_bytes([hash_results.to_bytes()[0], hash_results.to_bytes()[1]]);

    let defense_rand = u16::from_le_bytes([hash_results.to_bytes()[2], hash_results.to_bytes()[3]]);

    let knowledge_rand = u16::from_le_bytes([hash_results.to_bytes()[4], hash_results.to_bytes()[5]]);

    // range for stats in general is 40 - 140, in such case:
    // modulo formula is (rand%n) + min
    // where n = (max - min) + 1
   
    let (attack, defense, knowledge) = match class {
        WarriorClass::Validator => {
        // Balanced fighter - good at everything (60-99 range)
        //1) attack is of range 60 - 100
        //2) def is 40 - 100 and know is 40 - 80
        let attack = (attack_rand%41) + 60;
        let defense = (defense_rand%61) + 40;
        let knowledge = (knowledge_rand%41) + 40;
         (attack, defense, knowledge)
        },
        WarriorClass::Oracle => {
        let attack = (attack_rand%51) + 50;
        let defense = (defense_rand%41) + 40;
        let knowledge = (knowledge_rand%42) + 100;
         (attack, defense, knowledge)
        },
        WarriorClass::Guardian =>{
        let attack = (attack_rand%22) + 40;
        let defense = (defense_rand%41) + 100;
        let knowledge = (knowledge_rand%51) + 50;
         (attack, defense, knowledge)
        }, 

        WarriorClass::Daemon => {
        let attack = (attack_rand%41) + 100;
        let defense = (defense_rand%22) + 40;
        let knowledge = (knowledge_rand%51) + 50;
         (attack, defense, knowledge)
        }      
    };

        msg!(
        "⚔️ Combat Profile: ATK {} | DEF {} | KNOW {} | Strategy: {}",
        attack,
        defense, 
        knowledge,
        match class {
            WarriorClass::Validator => "Balanced fighter",
            WarriorClass::Oracle => "Knowledge specialist", 
            WarriorClass::Guardian => "Tank defender",
            WarriorClass::Daemon => "Glass cannon",
        }
    );




Ok((attack, defense, knowledge))
}