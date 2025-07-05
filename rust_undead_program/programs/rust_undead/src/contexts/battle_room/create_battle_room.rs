use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::helpers::is_warrior_ready;
use crate::error::RustUndeadError;

#[derive(Accounts)]
#[instruction(room_id: [u8; 32], warrior_name: String, selected_concepts: [u8; 5], selected_topics: [u8; 10], 
selected_questions: [u16; 10], correct_answers: [bool; 10]
)]
pub struct CreateBattleRoom<'info> {
    #[account(mut)]
    pub player_a: Signer<'info>,

    #[account(
        mut,
        seeds = [UNDEAD_WARRIOR, player_a.key().as_ref(), warrior_name.as_bytes()],
        constraint = warrior_a.owner == player_a.key() @ RustUndeadError::NotWarriorOwner,
        constraint = is_warrior_ready(&warrior_a) @ RustUndeadError::WarriorOnCooldown,
        bump
    )]
    pub warrior_a: Account<'info, UndeadWarrior>,

    #[account(
        init,
        payer = player_a,
        space = ANCHOR_DISCRIMINATOR + BattleRoom::INIT_SPACE,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
    )]
    pub battle_room: Account<'info, BattleRoom>,

    pub system_program: Program<'info, System>,
}

/// Create battle room with pre-selected educational content

impl<'info> CreateBattleRoom<'info> {
    pub fn create_battle_room(
        &mut self,
        room_id: [u8; 32],
        _warrior_name: String,
        selected_concepts: [u8; 5], 
        selected_topics: [u8; 10], 
        selected_questions: [u16; 10], 
        correct_answers: [bool; 10],
        bumps: &CreateBattleRoomBumps
    ) -> Result<()> {
        for &concept in &selected_concepts {
            require!(concept >= 1 && concept <= 10, RustUndeadError::InvalidConceptSelection);
        }
        
        // Ensure concepts are unique
        let mut unique_concepts = selected_concepts;
        unique_concepts.sort();
        for i in 1..unique_concepts.len() {
            require!(unique_concepts[i] != unique_concepts[i-1], RustUndeadError::InvalidConceptSelection);
        }

        self.battle_room.set_inner(
            BattleRoom { 
                room_id, 
                created_at: Clock::get()?.unix_timestamp, 
                player_a: self.player_a.key(), 
                player_b: None, 
                warrior_a: self.warrior_a.key(), 
                warrior_b: None, 
                selected_concepts, 
                selected_topics,  
                selected_questions,  
                correct_answers, 
                state: BattleState::QuestionsSelected, 
                player_a_ready: false, 
                player_b_ready: false, 
                current_question: 0, 
                player_a_answers: [None; 10], 
                player_b_answers: [None; 10], 
                player_a_correct: 0, 
                player_b_correct: 0, 
                winner: None, 
                battle_duration: 0, 
                bump: bumps.battle_room,
                battle_start_time: 0,
            }
        );

        msg!("🎮 Battle room created by Player A: {}", self.player_a.key());
        msg!("⚔️ Warrior selected: {} ({})", self.warrior_a.name, self.warrior_a.key());
        msg!("📚 Educational content ready:");
        msg!("   Concepts ({} selected): {:?}", selected_concepts.len(), selected_concepts);
        msg!("   Topics ({} selected): {:?}", selected_topics.len(), selected_topics);
        msg!("   Questions ({} selected): {:?}", selected_questions.len(), selected_questions);
        msg!("   Correct answers: {:?}", correct_answers);
        msg!("✅ Room ID: {:?}", room_id);
        msg!("✅ Room ready for Player B to join!");
        
        Ok(())
    }
}