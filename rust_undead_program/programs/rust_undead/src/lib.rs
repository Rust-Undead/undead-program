pub mod constants;
pub mod error;
pub mod contexts;
pub mod state;
pub mod helpers;

use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use ephemeral_rollups_sdk::anchor::ephemeral;


pub use constants::*;
pub use contexts::*;
pub use error::*;
pub use state::*;
pub use helpers::*;

declare_id!("BJkz2oAvir58MU5AJsj7usm248iy1mPnBM1hv2TVzqxE");


#[ephemeral]
#[program]
pub mod rust_undead {
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
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 60, 100);   // --average
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 60, 100);  // --average
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 80); // --average
            (attack, defense, knowledge)
        },
        WarriorClass::Oracle => {
            // Knowledge specialist - high knowledge, moderate combat
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 50, 100);   // --mid low
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 80);  //  --mid low
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141); // 100-140 --max
            (attack, defense, knowledge)
        },
        WarriorClass::Guardian => {
            // Tank - high defense, lower attack, good knowledge
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 60);    // 40-70 --low
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141); // 100-140 --max
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 50, 100); // 70-110 --average
            (attack, defense, knowledge)
        },
        WarriorClass::Daemon => {
            // Glass cannon - high attack, lower defense, good knowledge  
            let attack = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 100, 141);  // 100-140 --max
            let defense = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 40, 60);   // 40-80 --low
            let knowledge = ephemeral_vrf_sdk::rnd::random_u8_with_range(&randomness, 50, 100); // 70-110 --average
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

//answers to questions

pub fn answer_question(
    ctx: Context<AnswerQuestion>,
    room_id: [u8; 32],
    answer: bool,
    client_seed: u8,
) -> Result<()> {
    ctx.accounts.answer_question(room_id, answer, client_seed)
}

 
pub fn settle_battle_room(
    ctx: Context<EndBattleRoom>,
    room_id: [u8; 32],
) -> Result<()> {
    ctx.accounts.end_battle_room(room_id)
}

pub fn undelegate_battle_room(
    ctx: Context<UndelegateBattleRoom>,
    room_id: [u8; 32],
) -> Result<()> {
    ctx.accounts.undelegate_battle_room(room_id)
}

// cancel battle room if no one joined 
pub fn cancel_empty_battle_room(
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