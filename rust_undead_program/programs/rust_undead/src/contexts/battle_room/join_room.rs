use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::helpers::*;
use crate::error::RustUndeadError;

#[derive(Accounts)]
#[instruction(room_id: [u8; 32], warrior_name: String)]
pub struct JoinBattleRoom<'info> {
    #[account(mut)]
    pub player_b: Signer<'info>,

    #[account(
        mut,
        seeds = [UNDEAD_WARRIOR, player_b.key().as_ref(), warrior_name.as_bytes()],
        constraint = warrior_b.owner == player_b.key() @ RustUndeadError::NotWarriorOwner,
        constraint = is_warrior_ready(&warrior_b) @ RustUndeadError::WarriorOnCooldown,
        bump
    )]
    pub warrior_b: Account<'info, UndeadWarrior>,

    #[account(
        mut,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
        constraint = battle_room.room_id == room_id @ RustUndeadError::InvalidRoomId,
        constraint = battle_room.state == BattleState::QuestionsSelected @ RustUndeadError::InvalidBattleState,
        constraint = battle_room.player_b.is_none() @ RustUndeadError::BattleRoomFull,
        constraint = battle_room.player_a != player_b.key() @ RustUndeadError::NotAuthorized,
    )]
    pub battle_room: Account<'info, BattleRoom>,
}

/// Join an existing battle room
/// 
/// Requirements:
/// - Room must be in QuestionsSelected state (ready for players)
/// - Room must not be full (player_b slot empty)
/// - Warrior must be off cooldown
/// - Cannot join your own room
/// 
/// Data:
/// - room_id: [u8; 32] Room to join
/// - warrior_name: String Warrior to use in battle

impl<'info> JoinBattleRoom<'info> {
    pub fn join_battle_room(
        &mut self,
        room_id: [u8; 32],
        _warrior_name: String,
    ) -> Result<()> {
        let battle_room = &mut self.battle_room;
        let warrior_b = &self.warrior_b;
        let player_b = &self.player_b;
        
        // ✅ Validate warrior ownership and readiness
        require!(
            battle_room.room_id == room_id,
            RustUndeadError::InvalidRoomId
        );
        
        require!(
            warrior_b.key() != battle_room.warrior_a,
            RustUndeadError::SameWarriorCannotBattle
        );
       
        // ✅ Join the battle room
        battle_room.player_b = Some(player_b.key());
        battle_room.warrior_b = Some(warrior_b.key());
        battle_room.player_b_ready = false; // Initialize as not ready
        
        // ✅ Log successful join
        msg!("🎮 Player B joined the battle room!");
        msg!("⚔️ Warrior selected: {} ({})", warrior_b.name, warrior_b.key());
        msg!("👥 Battle participants:");
        msg!("   Player A: {} with {}", battle_room.player_a, battle_room.warrior_a);
        msg!("   Player B: {} with {}", player_b.key(), warrior_b.key());
        msg!("📚 Educational content ready:");
        msg!("   Concepts: {:?}", battle_room.selected_concepts);
        msg!("   Questions: 10 total prepared");
        msg!("✅ Room complete! Players can now study content and signal ready for battle.");
        
        Ok(())
    }
}