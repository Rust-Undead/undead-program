use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;

#[derive(Accounts)]
#[instruction(room_id: [u8; 32])]
pub struct CancelBattleRoom<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
        constraint = battle_room.room_id == room_id @ RustUndeadError::InvalidRoomId,
        constraint = battle_room.player_a == player.key() @ RustUndeadError::OnlyCreatorCanCancel,
        constraint = battle_room.state != BattleState::InProgress @ RustUndeadError::BattleAlreadyStarted,
        constraint = battle_room.state != BattleState::Completed @ RustUndeadError::BattleAlreadyCompleted,
        constraint = battle_room.state != BattleState::Cancelled @ RustUndeadError::BattleAlreadyCancelled,
    )]
    pub battle_room: Account<'info, BattleRoom>,

    #[account(
        mut,
        constraint = warrior_a.key() == battle_room.warrior_a @ RustUndeadError::InvalidWarrior,
        constraint = warrior_a.owner == player.key() @ RustUndeadError::NotWarriorOwner,
    )]
    pub warrior_a: Account<'info, UndeadWarrior>,

    #[account(
        mut,
        constraint = warrior_b.key() == battle_room.warrior_b.unwrap_or_default() @ RustUndeadError::InvalidWarrior,
    )]
    pub warrior_b: Option<Account<'info, UndeadWarrior>>,
}

impl<'info> CancelBattleRoom<'info> {
    pub fn cancel_battle_room(
        &mut self,
        room_id: [u8; 32],
    ) -> Result<()> {
        let battle_room = &mut self.battle_room;
        let warrior_a = &mut self.warrior_a;
        
        // Determine cancellation stage
        let cancellation_stage = match battle_room.state {
            BattleState::QuestionsSelected => {
                if battle_room.player_b.is_none() {
                    "Room Created - No opponent yet"
                } else {
                    "Opponent joined but not ready"
                }
            },
            BattleState::ReadyForDelegation => "Both players ready - Pre-delegation",
            _ => return Err(RustUndeadError::CannotCancelAtThisStage.into()),
        };

        msg!("🚫 Cancelling battle room at stage: {}", cancellation_stage);
        msg!("   Room ID: {:?}", room_id);
        msg!("   Creator: {}", battle_room.player_a);

        // ✅ Release Player A's warrior from battle
        warrior_a.last_battle_at = 0;
        warrior_a.cooldown_expires_at = 0;
        msg!("✅ Released {} from battle", warrior_a.name);

        // ✅ Release Player B's warrior if they joined
        if let Some(warrior_b) = &mut self.warrior_b {
            if battle_room.player_b.is_some() {
                warrior_b.last_battle_at = 0;
                warrior_b.cooldown_expires_at = 0;
                msg!("✅ Released {} from battle", warrior_b.name);
            }
        }

        // ✅ Calculate timing and provide feedback
        let current_time = Clock::get()?.unix_timestamp;
        let room_age = current_time - battle_room.created_at;
        
        if room_age < 300 { // 5 minutes
            msg!("⚡ Quick cancellation - no penalties");
        } else if battle_room.player_b.is_some() {
            msg!("⚠️ Late cancellation after opponent joined");
           
        }

        // ✅ Mark battle room as cancelled
        battle_room.state = BattleState::Cancelled;
        battle_room.winner = None;
        battle_room.battle_duration = room_age as u32;

        // ✅ Log final cancellation summary
        msg!("🚫 Battle room cancelled successfully!");
        msg!("   Status: {}", match cancellation_stage {
            "Room Created - No opponent yet" => "Clean cancellation",
            "Opponent joined but not ready" => "Pre-ready cancellation", 
            "Both players ready - Pre-delegation" => "Pre-battle cancellation",
            _ => "Unknown",
        });
        
        let affected_players = if battle_room.player_b.is_some() { 2 } else { 1 };
        msg!("   Affected players: {}", affected_players);
        msg!("   Warriors released from battle commitment");
        msg!("   Room marked as cancelled and cleaned up");

        Ok(())
    }
}