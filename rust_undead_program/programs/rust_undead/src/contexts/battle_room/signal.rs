use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;

#[derive(Accounts)]
#[instruction(room_id: [u8; 32], warrior_name: String)]
pub struct SignalReady<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
        constraint = battle_room.state == BattleState::QuestionsSelected @ RustUndeadError::InvalidBattleState,
        constraint = battle_room.is_player_in_room(&player.key()) @ RustUndeadError::PlayerNotInRoom,
    )]
    pub battle_room: Account<'info, BattleRoom>,

    #[account(
        mut,
        seeds = [UNDEAD_WARRIOR, player.key().as_ref(), warrior_name.as_bytes()],
        bump,
        constraint = warrior.owner == player.key() @ RustUndeadError::NotWarriorOwner,
        constraint = 
            (player.key() == battle_room.player_a && warrior.key() == battle_room.warrior_a) ||
            (Some(player.key()) == battle_room.player_b && Some(warrior.key()) == battle_room.warrior_b)
            @ RustUndeadError::NotWarriorOwner,
    )]
    pub warrior: Account<'info, UndeadWarrior>,

    #[account(
        mut,
        constraint = warrior_a.key() == battle_room.warrior_a @ RustUndeadError::InvalidWarrior,
    )]
    pub warrior_a: Account<'info, UndeadWarrior>,

    #[account(
        mut,
        constraint = warrior_b.key() == battle_room.warrior_b.unwrap() @ RustUndeadError::InvalidWarrior,
    )]
    pub warrior_b: Account<'info, UndeadWarrior>,
}

impl<'info> SignalReady<'info> {
    pub fn signal_ready(
        &mut self,
        room_id: [u8; 32],
        warrior_name: String,
    ) -> Result<()> {

        let battle_room = &mut self.battle_room;
        let player = self.player.key();

         require!(
            battle_room.room_id == room_id,
            RustUndeadError::InvalidRoomId
        );
        
        // Set readiness for the correct player
        if player == battle_room.player_a {
            require!(!battle_room.player_a_ready, RustUndeadError::AlreadyReady);
            battle_room.player_a_ready = true;
            msg!("Player A is ready for battle with warrior: {}", warrior_name);
        } else if Some(player) == battle_room.player_b {
            require!(!battle_room.player_b_ready, RustUndeadError::AlreadyReady);
            battle_room.player_b_ready = true;
            msg!("Player B is ready for battle with warrior: {}", warrior_name);
        } else {
            return Err(RustUndeadError::PlayerNotInRoom.into());
        }
        
        // ✅ PREPARE BATTLE when both players are ready
        if battle_room.player_a_ready && battle_room.player_b_ready {
            msg!("Both players ready! Preparing battle...");
            
            // Initialize warrior HP for battle
            let warrior_a = &mut self.warrior_a;
            let warrior_b = &mut self.warrior_b;
            
            // Reset HP to max for battle
            warrior_a.current_hp = warrior_a.max_hp;
            warrior_b.current_hp = warrior_b.max_hp;
            
            // Reset/initialize battle state
            battle_room.current_question = 0;
            battle_room.player_a_answers = [None; 10];
            battle_room.player_b_answers = [None; 10];
            battle_room.player_a_correct = 0;
            battle_room.player_b_correct = 0;
            battle_room.battle_duration = 0;
            battle_room.winner = None;
        
            // Update state to ready for delegation
            battle_room.state = BattleState::ReadyForDelegation;
            
            msg!(
                "⚔️ Battle prepared! {} ({} HP) vs {} ({} HP)",
                warrior_a.name,
                warrior_a.current_hp,
                warrior_b.name,
                warrior_b.current_hp
            );
            
            msg!("🎯 Battle room ready for delegation to ephemeral rollup!");
        }

        Ok(())
    }
}