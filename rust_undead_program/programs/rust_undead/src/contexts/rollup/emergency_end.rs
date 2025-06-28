use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;

use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

//emergency undelegate and end battle
// This context allows an admin to emergency undelegate and end a battle room
// without going through the normal battle flow.
// It is intended for use in extreme situations where the battle cannot continue,
// such as a critical bug or exploit that prevents normal operation.
#[commit]
#[derive(Accounts)]
#[instruction(room_id: [u8; 32])]
pub struct EmergencyUndelegateAndEnd<'info> {
    #[account(
        mut,
        constraint = authority.key() == config.admin @ RustUndeadError::NotAuthorized,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
        constraint = battle_room.room_id == room_id @ RustUndeadError::InvalidRoomId,
    )]
    pub battle_room: Account<'info, BattleRoom>,

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

    #[account(
        mut,
        seeds = [CONFIG],
        bump,
    )]
    pub config: Account<'info, Config>,
}

impl<'info> EmergencyUndelegateAndEnd<'info> {
    pub fn emergency_undelegate_and_end(
        &mut self,
        room_id: [u8; 32],
    ) -> Result<()> {
        let battle_room = &mut self.battle_room;
        let warrior_a = &mut self.warrior_a;
        let warrior_b = &mut self.warrior_b;
        let config = &mut self.config;
        let authority = self.authority.key();

        // Validate admin authorization
        require!(
            authority == config.admin,
            RustUndeadError::NotAuthorized
        );

        let current_time = Clock::get()?.unix_timestamp;
        let battle_duration = current_time - battle_room.battle_start_time;

        msg!("🚨 EMERGENCY ADMIN INTERVENTION");
        msg!("   Admin: {}", authority);
        msg!("   Room ID: {:?}", room_id);
        msg!("   Battle Progress: Question {}/10", battle_room.current_question + 1);
        msg!("   Duration: {} seconds", battle_duration);

        // Cancel Battle as No Contest
        battle_room.winner = None; // No winner - emergency cancellation
        battle_room.state = BattleState::Completed;
        battle_room.battle_duration = battle_duration as u32;

        msg!("🏆 Emergency Outcome: NO CONTEST - Battle Cancelled");

        //Reset Warriors to Full Health (No Penalties)
        warrior_a.current_hp = warrior_a.max_hp;
        warrior_b.current_hp = warrior_b.max_hp;

        // Apply minimal cooldown for emergency situations
        let emergency_cooldown = config.cooldown_time / 8; // 12.5% of normal cooldown
        
        warrior_a.last_battle_at = current_time;
        warrior_a.cooldown_expires_at = current_time + emergency_cooldown as i64;
        warrior_b.last_battle_at = current_time;
        warrior_b.cooldown_expires_at = current_time + emergency_cooldown as i64;

        msg!("⚕️ Warriors fully healed - emergency cooldown: {} seconds", emergency_cooldown);

        // Log Emergency Statistics
        msg!("📊 Emergency Battle Statistics:");
        msg!("   Final State: CANCELLED (No Contest)");
        msg!("   Questions Completed: {}/10", battle_room.current_question);
        msg!("   Final Scores - A: {}, B: {}", battle_room.player_a_correct, battle_room.player_b_correct);
        msg!("   Warriors Reset - {}: {}/100 HP, {}: {}/100 HP", 
            warrior_a.name, warrior_a.current_hp,
            warrior_b.name, warrior_b.current_hp
        );

        // Emergency Undelegate All Accounts
        commit_and_undelegate_accounts(
            &self.authority,
            vec![
                &self.battle_room.to_account_info(),
                &self.warrior_a.to_account_info(),
                &self.warrior_b.to_account_info(),
            ],
            &self.magic_context,
            &self.magic_program,
        )?;

        msg!("🚨 EMERGENCY INTERVENTION COMPLETE");
        msg!("✅ All accounts undelegated from rollup");
        msg!("🚫 Battle cancelled as no contest - no winner declared");
        msg!("⚕️ Warriors healed and on minimal cooldown until: {}", current_time + emergency_cooldown as i64);
        msg!("⚠️ No XP, achievements, or leaderboard changes applied");

        Ok(())
    }
}