use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;

#[commit]
#[derive(Accounts)]
#[instruction(room_id: [u8; 32])]
pub struct StartBattle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
        constraint = battle_room.state == BattleState::ReadyForDelegation @ RustUndeadError::InvalidBattleState,
        constraint = battle_room.player_a_ready && battle_room.player_b_ready @ RustUndeadError::PlayerNotReady,
    )]
    pub battle_room: Account<'info, BattleRoom>,

    #[account(
        mut,
        constraint = warrior_a.key() == battle_room.warrior_a @ RustUndeadError::InvalidWarrior,
        constraint = warrior_a.current_hp > 0 @ RustUndeadError::WarriorDefeated,
    )]
    pub warrior_a: Account<'info, UndeadWarrior>,

    #[account(
        mut,
        constraint = warrior_b.key() == battle_room.warrior_b.unwrap() @ RustUndeadError::InvalidWarrior,
        constraint = warrior_b.current_hp > 0 @ RustUndeadError::WarriorDefeated,
    )]
    pub warrior_b: Account<'info, UndeadWarrior>,
}

impl<'info> StartBattle<'info> {
    pub fn start_battle(
        &mut self,
        room_id: [u8; 32],
    ) -> Result<()> {
        let battle_room = &mut self.battle_room;
        let warrior_a = &mut self.warrior_a;
        let warrior_b = &mut self.warrior_b;
        
        // Validate room ID
        require!(
            battle_room.room_id == room_id,
            RustUndeadError::InvalidRoomId
        );
        
        // Final validation before battle starts
        require!(
            battle_room.winner.is_none(),
            RustUndeadError::BattleAlreadySettled
        );
        
        // Ensure warriors are at full HP
        require!(
            warrior_a.current_hp == warrior_a.max_hp,
            RustUndeadError::WarriorOnCooldown
        );
        require!(
            warrior_b.current_hp == warrior_b.max_hp,
            RustUndeadError::WarriorOnCooldown
        );

        msg!("Starting battle in room: {:?}", room_id);   
        
        // Initialize battle timing
        battle_room.state = BattleState::InProgress;
        let current_time = Clock::get()?.unix_timestamp;
        
        // Reset battle counters
        battle_room.current_question = 0;
        battle_room.player_a_correct = 0;
        battle_room.player_b_correct = 0;
        battle_room.battle_duration = 0;
        battle_room.battle_start_time = current_time;

        // Log battle start
        msg!("🔥 BATTLE BEGINS!");
        msg!(
            "⚔️  {} ({} HP) vs {} ({} HP) ⚔️",
            warrior_a.name,
            warrior_a.current_hp,
            warrior_b.name,
            warrior_b.current_hp
        );
        msg!(
            "📚 Questions ready: {} total questions from {} concepts",
            battle_room.selected_questions.len(),
            battle_room.selected_concepts.len()
        );
        msg!(
            "🎯 First question index: {} - Battle has begun!",
            battle_room.current_question
        );

        // Commit changes to the rollup
        commit_accounts(
            &self.authority, 
            vec![
                &self.battle_room.to_account_info(),
                &self.warrior_a.to_account_info(),
                &self.warrior_b.to_account_info(),
            ],
            &self.magic_context,
            &self.magic_program,
        )?;
        
        msg!("✅ Battle state committed to rollup successfully!");
        
        Ok(())
    }
}