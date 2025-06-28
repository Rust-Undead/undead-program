pub mod constants;
pub mod error;
pub mod contexts;
pub mod state;
pub mod helpers;

use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_rollups_sdk::anchor::ephemeral;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;


pub use constants::*;
pub use contexts::*;
pub use error::*;
pub use state::*;
pub use helpers::*;

declare_id!("9aVsYoGKsTMBTCEZ2K2UCfUJRV6X7PCCrz8txENGuJ3d");

#[ephemeral]
#[program]
pub mod rust_undead {
    use crate::instruction::UpdateFinalState;

    use super::*;

    // initialize game
pub fn initialize(
    ctx: Context<Initialize>,
	cooldown_time: u64,
) -> Result<()> {
    ctx.accounts.initialize(cooldown_time, &ctx.bumps)
}
// create the warrior ix
pub fn create_warrior(
    ctx: Context<CreateWarrior>,
    name: String, 
    dna: [u8; 8], 
    class: WarriorClass,
    client_seed: u8,
) -> Result<()> {
        // Validation. 
    require!(name.len() <= 32, RustUndeadError::NameTooLong);
    require!(name.len() > 0, RustUndeadError::NameEmpty);

    let warrior = &mut ctx.accounts.warrior;
    warrior.name = name;
    warrior.owner = ctx.accounts.player.key();
    warrior.dna = dna;
    warrior.created_at = Clock::get()?.unix_timestamp;
    warrior.warrior_class = class;
	warrior.last_battle_at = 0;
	warrior.cooldown_expires_at = 0;
	warrior.bump = ctx.bumps.warrior;

// Initialize battle stats (will be set by VRF callback)
    warrior.base_attack = 0;
    warrior.base_defense = 0;
    warrior.base_knowledge = 0;
    warrior.max_hp = 100;       
    warrior.current_hp = 100;
    warrior.battles_won = 0;
    warrior.battles_lost = 0;
    warrior.experience_points = 0;
    warrior.level = 1;

// Update user profile
    let user_profile = &mut ctx.accounts.user_profile;
    if user_profile.owner == Pubkey::default() {
// First time initialization
    user_profile.owner = ctx.accounts.player.key();
    user_profile.join_date = Clock::get()?.unix_timestamp;
    user_profile.warriors_created = 1;
    user_profile.story_level = 0;
    user_profile.total_battles_won = 0;
    user_profile.total_battles_lost = 0;
    user_profile.total_battles_fought = 0;
    user_profile.total_points = 0;
	user_profile.bump = ctx.bumps.user_profile;
} else {
    // Increment warrior count
    user_profile.warriors_created = user_profile.warriors_created.saturating_add(1);
}
        
// Update achievements
    let user_achievements = &mut ctx.accounts.user_achievements;
    if user_achievements.owner == Pubkey::default() {
        // First time initialization
        user_achievements.owner = ctx.accounts.player.key();
        user_achievements.overall_achievements = AchievementLevel::None;
        user_achievements.warrior_achivement = AchievementLevel::Bronze; // First warrior
        user_achievements.winner_achievement = AchievementLevel::None;
        user_achievements.battle_achievement = AchievementLevel::None;
        user_achievements.first_warrior_date = Clock::get()?.unix_timestamp;
        user_achievements.first_victory_date = 0;
		user_achievements.bump = ctx.bumps.user_achievements;

// Set initial warrior achievement based on first warrior creation
		user_achievements.warrior_achivement = calculate_warrior_achievement(user_profile.warriors_created);
        } else {
				
         // Update warrior achievement based on count
        user_achievements.warrior_achivement = calculate_warrior_achievement(user_profile.warriors_created)
        }
        
        // Update overall points and achievements
        user_profile.total_points = user_profile.total_points.saturating_add(100);
        user_achievements.overall_achievements = calculate_overall_achievement(user_profile.total_points);
        
        msg!("Warrior '{}' created with 100 HP, requesting VRF for combat stats...", warrior.name);

        // VRF for combat stats generation
        let ix = create_request_randomness_ix(RequestRandomnessParams { 
            payer: ctx.accounts.player.key(), 
            oracle_queue: ctx.accounts.oracle_queue.key(), 
            callback_program_id: ID, 
            callback_discriminator: instruction::CallbackWarriorStats::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32], 
            accounts_metas: Some(vec![SerializableAccountMeta{
                pubkey: ctx.accounts.warrior.key(),
                is_signer: false,
                is_writable: true
            }]), 
            ..Default::default()
        });

        ctx.accounts.invoke_signed_vrf(&ctx.accounts.player.to_account_info(), &ix)?;
        Ok(())
    }

pub fn callback_warrior_stats(
  ctx: Context<CallbackWarriorStats>,
  randomness: [u8; 32],
) -> Result<()> {
    let warrior = &mut ctx.accounts.warrior;
    let class = warrior.warrior_class;
    
    msg!("Generating random combat stats for warrior: {} (class: {:?})", warrior.name, class);
    
    // Generate stats based on class specialization
    // Stats range: 40-140 (100 point spread for strategic variety)
    // HP range: 0-100 (battle mechanic - reduce to 0 to win)

    let (attack, defense, knowledge) = match class {
        WarriorClass::Validator => {
            // Balanced fighter - good at everything (70-110 range)
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 70, 111);   // --average
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 70, 111);  // --average
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 70, 111); // --average
            (attack, defense, knowledge)
        },
        WarriorClass::Oracle => {
            // Knowledge specialist - high knowledge, moderate combat
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 60, 101);   // --mid low
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 60, 101);  //  --mid low
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141); // 100-140 --max
            (attack, defense, knowledge)
        },
        WarriorClass::Guardian => {
            // Tank - high defense, lower attack, good knowledge
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 70);    // 40-70 --low
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141); // 100-140 --max
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 70, 111); // 70-110 --average
            (attack, defense, knowledge)
        },
        WarriorClass::Daemon => {
            // Glass cannon - high attack, lower defense, good knowledge  
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141);  // 100-140 --max
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 70);   // 40-80 --low
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 70, 111); // 70-110 --average
            (attack, defense, knowledge)
        },
    };
    
    // Set the generated combat stats
    warrior.base_attack = attack as u16;
    warrior.base_defense = defense as u16;
    warrior.base_knowledge = knowledge as u16;
    // HP is fixed at 100 for all warriors during creation
    
    msg!(
        "✅ Warrior '{}' combat stats finalized - ATK: {}, DEF: {}, KNOW: {}, HP: {}/{}",
        warrior.name,
        warrior.base_attack,
        warrior.base_defense,
        warrior.base_knowledge,
        warrior.current_hp,
        warrior.max_hp
    );
    
    msg!(
        "🎯 Class: {:?} | Ready for strategic 0-100 HP battles!",
        warrior.warrior_class
    );
    
    msg!(
        "⚔️ Combat Profile: ATK {} | DEF {} | KNOW {} | Strategy: {}",
        warrior.base_attack,
        warrior.base_defense, 
        warrior.base_knowledge,
        match class {
            WarriorClass::Validator => "Balanced fighter",
            WarriorClass::Oracle => "Knowledge specialist", 
            WarriorClass::Guardian => "Tank defender",
            WarriorClass::Daemon => "Glass cannon",
        }
    );
    
    Ok(())
}

//create battle room
pub fn create_battle_room(
  ctx: Context<CreateBattleRoom>,
  room_id: [u8; 32],
  warrior_name: String,
  selected_concepts: [u8; 5],
  selected_questions: [u16; 10],
  correct_answers: [bool; 10],
) -> Result<()> {
ctx.accounts.create_battle_room(
		room_id, 
		warrior_name, 
		selected_concepts, 
		selected_questions, 
		correct_answers, 
		&ctx.bumps
	)
}

// join battle room 
pub fn join_battle_room(
    ctx: Context<JoinBattleRoom>,
    room_id: [u8; 32],
    warrior_name: String,
) -> Result<()> {
ctx.accounts.join_battle_room(room_id, warrior_name)
}

//signal ready
pub fn signal_ready(
    ctx: Context<SignalReady>,
    room_id: [u8; 32],
    warrior_name: String,
) -> Result<()> {
ctx.accounts.signal_ready(room_id, warrior_name)
}

//delegate to rollup
pub fn delegate_battle(
    ctx: Context<DelegateBattle>,
    room_id: [u8; 32],
    player_a: Pubkey,
    warrior_a_name: String,
    player_b: Pubkey,
    warrior_b_name: String,
) -> Result<()> {
ctx.accounts.delegate_to_rollup(
    room_id, 
    player_a, 
    warrior_a_name, 
    player_b, 
    warrior_b_name
)
}
//start battle
pub fn start_battle(
    ctx: Context<StartBattle>,
    room_id: [u8; 32],
) -> Result<()> {
ctx.accounts.start_battle(room_id)
}

//cancel battle
pub fn cancel_battle(
    ctx: Context<CancelBattleRoom>,
    room_id: [u8; 32],
) -> Result<()> {
ctx.accounts.cancel_battle_room(room_id)
}


// player answers and battle action 
pub fn answer_question(
    ctx: Context<AnswerQuestion>, 
    room_id: [u8; 32], 
    answer: bool, 
    client_seed: u8
) -> Result<()> {
// Validate room ID
    require!(ctx.accounts.battle_room.room_id == room_id, RustUndeadError::InvalidRoomId);

    let player = ctx.accounts.player.key();
    let current_q = ctx.accounts.battle_room.current_question as usize;

    require!(current_q < 10, RustUndeadError::InvalidQuestionIndex);
    require!(!ctx.accounts.battle_room.has_player_answered(&player, ctx.accounts.battle_room.current_question),
            RustUndeadError::AlreadyAnswered);
    
    // ✅ STEP 1: Record player's answer privately
    if ctx.accounts.battle_room.player_a == player {
        ctx.accounts.battle_room.player_a_answers[current_q] = Some(answer);
        msg!("🎯 Player A submitted answer for question {}", current_q + 1);
    } else if ctx.accounts.battle_room.player_b == Some(player) {
        ctx.accounts.battle_room.player_b_answers[current_q] = Some(answer);
        msg!("🎯 Player B submitted answer for question {}", current_q + 1);
    } else {
        return Err(RustUndeadError::PlayerNotInRoom.into());
    }
    
    // ✅ STEP 2: Check if both players have answered
    let both_answered = ctx.accounts.battle_room.player_a_answers[current_q].is_some() 
        && ctx.accounts.battle_room.player_b_answers[current_q].is_some();
    
    if !both_answered {
        // Only one player has answered - wait for opponent
        msg!("⏳ Answer submitted! Waiting for opponent to answer question {}", current_q + 1);
        return Ok(());
    }

    // ✅ STEP 3: Both players answered - reveal and process results
    msg!("🎭 Both players submitted! Revealing answers...");
    
    let player_a_answer = ctx.accounts.battle_room.player_a_answers[current_q].unwrap();
    let player_b_answer = ctx.accounts.battle_room.player_b_answers[current_q].unwrap();
    let correct_answer = ctx.accounts.battle_room.correct_answers[current_q];
    
    // Reveal both answers simultaneously
    msg!("📊 Question {} Results:", current_q + 1);
    msg!("   Player A answered: {} ({})", 
        player_a_answer, 
        if player_a_answer == correct_answer { "✅ CORRECT" } else { "❌ WRONG" }
    );
    msg!("   Player B answered: {} ({})", 
        player_b_answer, 
        if player_b_answer == correct_answer { "✅ CORRECT" } else { "❌ WRONG" }
    );
    msg!("   Correct answer was: {}", correct_answer);
    
    // ✅ STEP 4: Process answers and update scores
    let player_a_correct = player_a_answer == correct_answer;
    let player_b_correct = player_b_answer == correct_answer;
    
    if player_a_correct {
        ctx.accounts.battle_room.player_a_correct += 1;
        msg!("⚔️ Player A gets point!");
    }
    
    if player_b_correct {
        ctx.accounts.battle_room.player_b_correct += 1;
        msg!("⚔️ Player B gets point!");
    }
    
    msg!("📊 Updated Scores - A: {}, B: {}", 
        ctx.accounts.battle_room.player_a_correct, ctx.accounts.battle_room.player_b_correct);
    
    // ✅ STEP 5: Handle VRF damage calculation for each correct answer
    if player_a_correct {
        msg!("🎲 Player A correct - triggering VRF damage to Player B's warrior...");
        
        let (attacker_key, defender_key) = if ctx.accounts.battle_room.warrior_a == ctx.accounts.attacker_warrior.key() {
            (ctx.accounts.battle_room.warrior_a, ctx.accounts.battle_room.warrior_b.unwrap())
        } else {
            (ctx.accounts.battle_room.warrior_b.unwrap(), ctx.accounts.battle_room.warrior_a)
        };
        
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: ctx.accounts.player.key(),
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::CallbackDamage::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed; 32],
            accounts_metas: Some(vec![
                SerializableAccountMeta {
                    pubkey: ctx.accounts.battle_room.key(),
                    is_signer: false,
                    is_writable: true,
                },
                SerializableAccountMeta {
                    pubkey: attacker_key,
                    is_signer: false,
                    is_writable: true,
                },
                SerializableAccountMeta {
                    pubkey: defender_key,
                    is_signer: false,
                    is_writable: true,
                },
            ]),
            ..Default::default()
        });

        ctx.accounts.invoke_signed_vrf(&ctx.accounts.player.to_account_info(), &ix)?;
        msg!("⚔️ Player A damage VRF triggered!");
    }
    
    if player_b_correct {
        msg!("🎲 Player B correct - triggering VRF damage to Player A's warrior...");
        
        let (attacker_key, defender_key) = if ctx.accounts.battle_room.warrior_b.unwrap() == ctx.accounts.attacker_warrior.key() {
            (ctx.accounts.battle_room.warrior_b.unwrap(), ctx.accounts.battle_room.warrior_a)
        } else {
            (ctx.accounts.battle_room.warrior_a, ctx.accounts.battle_room.warrior_b.unwrap())
        };
        
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: ctx.accounts.player.key(),
            oracle_queue: ctx.accounts.oracle_queue.key(),
            callback_program_id: ID,
            callback_discriminator: instruction::CallbackDamage::DISCRIMINATOR.to_vec(),
            caller_seed: [client_seed.wrapping_add(1); 32],
            accounts_metas: Some(vec![
                SerializableAccountMeta {
                    pubkey: ctx.accounts.battle_room.key(),
                    is_signer: false,
                    is_writable: true,
                },
                SerializableAccountMeta {
                    pubkey: attacker_key,
                    is_signer: false,
                    is_writable: true,
                },
                SerializableAccountMeta {
                    pubkey: defender_key,
                    is_signer: false,
                    is_writable: true,
                },
            ]),
            ..Default::default()
        });

        ctx.accounts.invoke_signed_vrf(&ctx.accounts.player.to_account_info(), &ix)?;
        msg!("⚔️ Player B damage VRF triggered!");
    }
    
    if !player_a_correct && !player_b_correct {
        msg!("❌ Neither player correct - no damage dealt, moving to next question");
    }
    
    // ✅ STEP 6: Advance question or end battle
    if current_q < 9 && ctx.accounts.battle_room.state != BattleState::Completed {
        ctx.accounts.battle_room.current_question += 1;
        msg!("📋 Moving to question {}", ctx.accounts.battle_room.current_question + 1);
    } else if current_q == 9 && ctx.accounts.battle_room.state != BattleState::Completed {
        msg!("🏁 All questions completed! Determining final winner by HP...");
        
        let warrior_a_hp = if ctx.accounts.battle_room.warrior_a == ctx.accounts.attacker_warrior.key() {
            ctx.accounts.attacker_warrior.current_hp
        } else {
            ctx.accounts.defender_warrior.current_hp
        };
        let warrior_b_hp = if ctx.accounts.battle_room.warrior_b.unwrap() == ctx.accounts.attacker_warrior.key() {
            ctx.accounts.attacker_warrior.current_hp
        } else {
            ctx.accounts.defender_warrior.current_hp
        };
        
        if warrior_a_hp > warrior_b_hp {
            ctx.accounts.battle_room.winner = Some(ctx.accounts.battle_room.player_a);
            msg!("🏆 Player A wins with {} HP vs {} HP!", warrior_a_hp, warrior_b_hp);
        } else if warrior_b_hp > warrior_a_hp {
            ctx.accounts.battle_room.winner = Some(ctx.accounts.battle_room.player_b.unwrap());
            msg!("🏆 Player B wins with {} HP vs {} HP!", warrior_b_hp, warrior_a_hp);
        } else {
            let (score_a, score_b) = ctx.accounts.battle_room.get_scores();
            if score_a > score_b {
                ctx.accounts.battle_room.winner = Some(ctx.accounts.battle_room.player_a);
                msg!("🏆 HP tied at {}! Player A wins by score: {} vs {}", warrior_a_hp, score_a, score_b);
            } else if score_b > score_a {
                ctx.accounts.battle_room.winner = Some(ctx.accounts.battle_room.player_b.unwrap());
                msg!("🏆 HP tied at {}! Player B wins by score: {} vs {}", warrior_b_hp, score_b, score_a);
            } else {
                ctx.accounts.battle_room.winner = Some(ctx.accounts.battle_room.player_a);
                msg!("🏆 Perfect tie! HP: {}, Score: {} each - Player A wins by default", warrior_a_hp, score_a);
            }
        }
        ctx.accounts.battle_room.state = BattleState::Completed;
    }
    
    // ✅ STEP 7: Update battle timing
    let current_time = Clock::get()?.unix_timestamp;
    ctx.accounts.battle_room.battle_duration = (current_time - ctx.accounts.battle_room.battle_start_time) as u32;

    // ✅ STEP 8: Only commit when battle is complete
    if ctx.accounts.battle_room.state == BattleState::Completed {
        commit_accounts(
            &ctx.accounts.player,
            vec![
                &ctx.accounts.battle_room.to_account_info(),
                &ctx.accounts.attacker_warrior.to_account_info(),
                &ctx.accounts.defender_warrior.to_account_info(),
            ],
            &ctx.accounts.magic_context,  
            &ctx.accounts.magic_program,
        )?;
        msg!("🏁 Battle completed! Final state committed to rollup for settlement");
    } else {
        msg!("⚔️ Battle continues... Question {} ready", ctx.accounts.battle_room.current_question + 1);
    }
   
    Ok(())
}

// vrf callback for attack and defense damage
pub fn callback_damage(
    ctx: Context<CallbackDamage>,
    randomness: [u8; 32],
) -> Result<()> {
    let battle_room = &mut ctx.accounts.battle_room;
    let attacker_warrior = &mut ctx.accounts.attacker_warrior;
    let defender_warrior = &mut ctx.accounts.defender_warrior;
    let current_q = battle_room.current_question as usize;

    // ✅ Determine damage range based on question phase
    let (min_damage, max_damage) = match current_q {
        0..=2 => (2, 10),    // Questions 1-3: Learning Phase
        3..=6 => (6, 15),   // Questions 4-7: Pressure Phase  
        7..=9 => (10, 20),  // Questions 8-10: Deadly Phase
        _ => (1, 1),        // Fallback
    };

    // ✅ Generate base damage using VRF
    let base_damage = ephemeral_vrf_sdk::rnd::random_u8_with_range(
        &randomness, 
        min_damage, 
        max_damage + 1
    );
    
    // ✅ Apply warrior stat modifiers
    let attack_bonus = attacker_warrior.base_attack as i32;
    let defense_reduction = defender_warrior.base_defense as i32;
    
    // Calculate modifier: (attack - defense) / 10, capped at +/-5
    let stat_modifier = ((attack_bonus - defense_reduction) / 10).clamp(-5, 5);
    
    // Apply modifier but ensure minimum 1 damage
    let final_damage = (base_damage as i32 + stat_modifier).max(1) as u16;

    // ✅ Apply damage to defender
    let old_hp = defender_warrior.current_hp;
    defender_warrior.current_hp = defender_warrior.current_hp.saturating_sub(final_damage);
    let new_hp = defender_warrior.current_hp;

    // ✅ Log damage calculation details
    msg!("🎲 VRF Damage Calculation:");
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
    msg!("   VRF Base Damage: {}", base_damage);
    msg!("   Attacker {} ATK: {}", attacker_warrior.name, attacker_warrior.base_attack);
    msg!("   Defender {} DEF: {}", defender_warrior.name, defender_warrior.base_defense);
    msg!("   Stat Modifier: {}", stat_modifier);
    msg!("   Final Damage: {}", final_damage);
    msg!("🩸 {} takes {} damage! HP: {} → {}", 
        defender_warrior.name, final_damage, old_hp, new_hp);

    // ✅ Check for elimination victory
    if defender_warrior.current_hp == 0 {
        battle_room.winner = Some(attacker_warrior.owner);
        battle_room.state = BattleState::Completed;
        msg!("💀 {} has been defeated!", defender_warrior.name);
        msg!("🏆 {} wins by elimination!", attacker_warrior.name);
        
        // Update battle timing
        let current_time = Clock::get()?.unix_timestamp;
        battle_room.battle_duration = (current_time - battle_room.battle_start_time) as u32;
    } else if defender_warrior.current_hp <= 10 {
        msg!("⚠️ {} is critically wounded! ({} HP remaining)", 
            defender_warrior.name, defender_warrior.current_hp);
    }

    // ✅ Update warrior battle experience (small XP gain for dealing damage)
    attacker_warrior.experience_points = attacker_warrior.experience_points.saturating_add(final_damage as u64);

    // ✅ Commit state if battle completed by elimination
    if battle_room.state == BattleState::Completed {
        commit_accounts(
            &ctx.accounts.vrf_program_identity,
            vec![
                &ctx.accounts.battle_room.to_account_info(),
                &ctx.accounts.attacker_warrior.to_account_info(),
                &ctx.accounts.defender_warrior.to_account_info(),
            ],
            &ctx.accounts.magic_context,
            &ctx.accounts.magic_program, 
        )?;
        msg!("🏁 Elimination victory! Battle state committed for settlement");
    } else {
        msg!("⚔️ Battle continues with {} at {} HP", defender_warrior.name, new_hp);
    }

    Ok(())
}

//undelegate battle to base layer 
pub fn undelegate_battle(
    ctx: Context<SettleBattleRoom>,
    room_id: [u8; 32],
) -> Result<()> {
    ctx.accounts.settle_battle_room(room_id)
}

// cancel battle room if no one joined 
pub fn end_battle_room(
    ctx:Context<CancelBattleRoom>,
    room_id: [u8; 32],
) -> Result<()> {
    ctx.accounts.cancel_battle_room(room_id)
}

// now on base layer, update final state 
pub fn update_final_state(
    ctx: Context<UpdateState>,
    room_id: [u8; 32],
) -> Result<()> {
    ctx.accounts.update_state(room_id)
}

// emergency cancel battle room
pub fn emergency_cancel_battle(
    ctx: Context<EmergencyUndelegateAndEnd>,
    room_id: [u8; 32],
) -> Result<()> {
    ctx.accounts.emergency_undelegate_and_end(room_id)
}

}



/// CONTEXTS //////
//create warrior 
#[vrf]
#[derive(Accounts)]
#[instruction( name: String, dna: [u8; 8], class: WarriorClass)]
pub struct CreateWarrior<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        init,
        payer = player,
        space = ANCHOR_DISCRIMINATOR + UndeadWarrior::INIT_SPACE,
        seeds = [UNDEAD_WARRIOR, player.key().as_ref(), name.as_bytes()],
        bump
    )]
    pub warrior: Account<'info, UndeadWarrior>,
    
    #[account(
        init_if_needed,
        payer = player,
        space = ANCHOR_DISCRIMINATOR + UserProfile::INIT_SPACE,
        seeds = [USER_PROFILE, player.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        init_if_needed,
        payer = player,
        space = ANCHOR_DISCRIMINATOR + UserAchievements::INIT_SPACE,
        seeds = [USER_ACHIEVEMENT, player.key().as_ref()],
        bump
    )]
    pub user_achievements: Account<'info, UserAchievements>,

		 /// CHECK: The oracle queue
		 #[account(
			mut,
			address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE
		 )]
		pub oracle_queue: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CallbackWarriorStats<'info> {
	/// This check ensure that the vrf_program_identity (which is a PDA) is a singer
  /// enforcing the callback is executed by the VRF program through CPI
#[account(
    address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY
)]
pub vrf_program_identity: Signer<'info>,
#[account(mut)]
pub warrior : Account<'info,UndeadWarrior>,
}


////// ROLLUP CONTEXTS //////
// 1. ANSWER QUESTION
#[vrf]
#[commit]
#[derive(Accounts)]
#[instruction(room_id: [u8; 32], answer: bool)]
pub struct AnswerQuestion<'info> {
#[account(mut)]
pub player: Signer<'info>,

//battle room account
#[account(
    mut,
    seeds = [BATTLE, room_id.as_ref()],
    bump,
    constraint = battle_room.room_id == room_id @ RustUndeadError::InvalidRoomId,
    constraint = battle_room.state == BattleState::InProgress @ RustUndeadError::InvalidBattleState,
    constraint = battle_room.current_question < 10 @ RustUndeadError::AllQuestionsAnswered,
    constraint = !battle_room.has_player_answered(&player.key(), battle_room.current_question) @ RustUndeadError::AlreadyAnswered,
    constraint = battle_room.is_player_in_room(&player.key()) @ RustUndeadError::PlayerNotInRoom,
    )]
pub battle_room: Account<'info, BattleRoom>,

//attacking warrior account
#[account(
    mut,
    constraint = attacker_warrior.owner == player.key() @ RustUndeadError::NotWarriorOwner,
    constraint = attacker_warrior.current_hp > 0 @ RustUndeadError::WarriorDefeated,
    constraint = (battle_room.player_a == player.key() && attacker_warrior.key() == battle_room.warrior_a) 
    ||
    (battle_room.player_b == Some(player.key()) && attacker_warrior.key() == battle_room.warrior_b.unwrap()) @ RustUndeadError::NotWarriorOwner,
    )]
    pub attacker_warrior: Account<'info, UndeadWarrior>,

//defending warrior account
#[account(
    mut,
    constraint = defender_warrior.current_hp > 0 @ RustUndeadError::WarriorDefeated,
    constraint = defender_warrior.key() != attacker_warrior.key() @ RustUndeadError::CannotAttackSelf,
    constraint = (battle_room.player_a == player.key() && defender_warrior.key() == battle_room.warrior_b.unwrap()) ||
    (battle_room.player_b == Some(player.key()) && defender_warrior.key() == battle_room.warrior_a) @ RustUndeadError::InvalidWarrior,
)]
    pub defender_warrior: Account<'info, UndeadWarrior>,

/// CHECK: The oracle queue
#[account(
    mut, 
    address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE
)]
pub oracle_queue: AccountInfo<'info>,
}


#[commit]
#[derive(Accounts)]
pub struct CallbackDamage<'info> {
#[account(
    address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY
)]
pub vrf_program_identity: Signer<'info>,

#[account(mut)]
pub battle_room: Account<'info, BattleRoom>,

#[account(mut)]
pub attacker_warrior: Account<'info, UndeadWarrior>,

#[account(mut)]
pub defender_warrior: Account<'info, UndeadWarrior>,
}



