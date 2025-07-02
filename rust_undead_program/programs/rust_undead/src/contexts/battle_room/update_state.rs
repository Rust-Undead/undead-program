use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;
use crate::helpers::*;

//after base battle is completed, update all state
#[derive(Accounts)]
#[instruction(room_id: [u8; 32])]
pub struct UpdateState<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
        constraint = battle_room.room_id == room_id @ RustUndeadError::InvalidRoomId,
        constraint = battle_room.state == BattleState::Completed @ RustUndeadError::InvalidBattleState,
        constraint = battle_room.winner.is_some() @ RustUndeadError::InvalidBattleState,
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
        seeds = [USER_PROFILE, warrior_a.owner.as_ref()],
        bump,
    )]
    pub profile_a: Account<'info, UserProfile>,

    #[account(
        mut,
        seeds = [USER_PROFILE, warrior_b.owner.as_ref()],
        bump,
    )]
    pub profile_b: Account<'info, UserProfile>,

    #[account(
        mut,
        seeds = [USER_ACHIEVEMENT, warrior_a.owner.as_ref()],
        bump,
    )]
    pub achievements_a: Account<'info, UserAchievements>,

    #[account(
        mut,
        seeds = [USER_ACHIEVEMENT, warrior_b.owner.as_ref()],
        bump,
    )]
    pub achievements_b: Account<'info, UserAchievements>,

    #[account(
        mut,
        seeds = [CONFIG, authority.key().as_ref()],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [LEADERBOARD, authority.key().as_ref()],
        bump,
    )]
    pub leaderboard: Account<'info, Leaderboard>,
}

impl<'info> UpdateState<'info> {
    pub fn update_state(
        &mut self,
        room_id: [u8; 32],
    ) -> Result<()> {
        let battle_room = &mut self.battle_room;
        let warrior_a = &mut self.warrior_a;
        let warrior_b = &mut self.warrior_b;
        let profile_a = &mut self.profile_a;
        let profile_b = &mut self.profile_b;
        let achievements_a = &mut self.achievements_a;
        let achievements_b = &mut self.achievements_b;
        let config = &mut self.config;
        let leaderboard = &mut self.leaderboard;

        //Check cooldown status for both warriors
        let current_time = Clock::get()?.unix_timestamp;
        
        let warrior_a_cooldown_remaining = if warrior_a.cooldown_expires_at > current_time {
            warrior_a.cooldown_expires_at - current_time
        } else {
            0
        };
        
        let warrior_b_cooldown_remaining = if warrior_b.cooldown_expires_at > current_time {
            warrior_b.cooldown_expires_at - current_time
        } else {
            0
        };

        msg!("⏱️ Warrior Cooldown Status:");
        msg!("   {} - {} seconds remaining", warrior_a.name, warrior_a_cooldown_remaining);
        msg!("   {} - {} seconds remaining", warrior_b.name, warrior_b_cooldown_remaining);

        // ✅ Check Warrior Battle Readiness (NO healing)
        msg!("⚔️ Current Warrior Status:");
        
        // Warrior A status
        let warrior_a_ready = warrior_a_cooldown_remaining == 0 && warrior_a.current_hp > 0;
        msg!("   {} - HP: {}/{}, Cooldown: {}s, Ready: {}", 
            warrior_a.name, 
            warrior_a.current_hp, 
            warrior_a.max_hp,
            warrior_a_cooldown_remaining,
            if warrior_a_ready { "Yes" } else { "No" }
        );

        // Warrior B status
        let warrior_b_ready = warrior_b_cooldown_remaining == 0 && warrior_b.current_hp > 0;
        msg!("   {} - HP: {}/{}, Cooldown: {}s, Ready: {}", 
            warrior_b.name, 
            warrior_b.current_hp, 
            warrior_b.max_hp,
            warrior_b_cooldown_remaining,
            if warrior_b_ready { "Yes" } else { "No" }
        );

        // ✅ Determine winner and loser
        let winner_player = battle_room.winner.unwrap();
        let is_player_a_winner = winner_player == battle_room.player_a;

        // Create references instead of moving the values
        let (winner_profile, loser_profile) = if is_player_a_winner {
            (&mut *profile_a, &mut *profile_b)
        } else {
            (&mut *profile_b, &mut *profile_a)
        };

        let (winner_achievements, loser_achievements) = if is_player_a_winner {
            (&mut *achievements_a, &mut *achievements_b)
        } else {
            (&mut *achievements_b, &mut *achievements_a)
        };

        // Keep references to the original warriors for later use
        let (winner_warrior, loser_warrior) = if is_player_a_winner {
            (&*warrior_a, &*warrior_b)
        } else {
            (&*warrior_b, &*warrior_a)
        };

        msg!("📈 Updating all state after battle settlement...");
        msg!("   Room ID: {:?}", room_id);
        msg!("   Winner: {} ({})", 
            if is_player_a_winner { "Player A" } else { "Player B" },
            winner_player
        );

        // ✅ Calculate XP amounts (matching settle_battle logic)
        let winner_score = if is_player_a_winner {
            battle_room.player_a_correct
        } else {
            battle_room.player_b_correct
        };

        let loser_score = battle_room.player_a_correct + battle_room.player_b_correct - winner_score;

        let winner_xp = 40 + (winner_score as u32 * 4); 
        let loser_xp = 20 + (loser_score as u32 * 2); 

        // ✅ Update User Profiles
        winner_profile.total_battles_won = winner_profile.total_battles_won.saturating_add(1);
        winner_profile.total_battles_fought = winner_profile.total_battles_fought.saturating_add(1);
        winner_profile.total_points = winner_profile.total_points.saturating_add(winner_xp);

        loser_profile.total_battles_lost = loser_profile.total_battles_lost.saturating_add(1);
        loser_profile.total_battles_fought = loser_profile.total_battles_fought.saturating_add(1);
        loser_profile.total_points = loser_profile.total_points.saturating_add(loser_xp);

        msg!("👤 Profile Updates:");
        msg!("   {} - Battles: {}, Wins: {}, Points: {}", 
            winner_warrior.name, winner_profile.total_battles_fought, 
            winner_profile.total_battles_won, winner_profile.total_points);
        msg!("   {} - Battles: {}, Losses: {}, Points: {}", 
            loser_warrior.name, loser_profile.total_battles_fought, 
            loser_profile.total_battles_lost, loser_profile.total_points);

        // ✅ Update Achievements
        winner_achievements.winner_achievement = calculate_winner_achievement(winner_profile.total_battles_won);
        winner_achievements.battle_achievement = calculate_battle_achievement(winner_profile.total_battles_fought);
        winner_achievements.overall_achievements = calculate_overall_achievement(winner_profile.total_points);

        loser_achievements.battle_achievement = calculate_battle_achievement(loser_profile.total_battles_fought);
        loser_achievements.overall_achievements = calculate_overall_achievement(loser_profile.total_points);

      

        msg!("🏅 Achievement Updates:");
        msg!("   {} - Winner: {:?}, Battle: {:?}, Overall: {:?}", 
            winner_warrior.name, winner_achievements.winner_achievement,
            winner_achievements.battle_achievement, winner_achievements.overall_achievements);
        msg!("   {} - Battle: {:?}, Overall: {:?}", 
            loser_warrior.name, loser_achievements.battle_achievement, 
            loser_achievements.overall_achievements);

        // ✅ Update Global Config Stats
        config.total_battles = config.total_battles.saturating_add(1);
        msg!("📊 Global Stats - Total Battles: {}", config.total_battles);

        // ✅ Update Leaderboard
        leaderboard.update_player_score(winner_warrior.owner, winner_profile.total_points)?;
        leaderboard.update_player_score(loser_warrior.owner, loser_profile.total_points)?;

        // Check for leaderboard achievements
        if leaderboard.is_top_10(winner_warrior.owner) {
            msg!("🌟 {} reached top 10 leaderboard!", winner_warrior.name);
        }

        if let Some(rank) = leaderboard.get_player_rank(winner_warrior.owner) {
            msg!("📈 {} leaderboard rank: #{}", winner_warrior.name, rank);
        }

        if let Some(rank) = leaderboard.get_player_rank(loser_warrior.owner) {
            msg!("📈 {} leaderboard rank: #{}", loser_warrior.name, rank);
        }

        // ✅ Battle Availability Messages
        if !warrior_a_ready {
            if warrior_a_cooldown_remaining > 0 {
                msg!("⏳ {} needs {} more seconds cooldown", warrior_a.name, warrior_a_cooldown_remaining);
            }
            if warrior_a.current_hp == 0 {
                msg!("💔 {} is defeated and needs healing", warrior_a.name);
            }
        }

        if !warrior_b_ready {
            if warrior_b_cooldown_remaining > 0 {
                msg!("⏳ {} needs {} more seconds cooldown", warrior_b.name, warrior_b_cooldown_remaining);
            }
            if warrior_b.current_hp == 0 {
                msg!("💔 {} is defeated and needs healing", warrior_b.name);
            }
        }

        // ✅ Final State Summary
        msg!("✅ Complete State Update Summary:");
        msg!("   📊 Profiles: Updated battle records and points");
        msg!("   🏅 Achievements: Recalculated all achievement levels");
        msg!("   🌍 Global Config: Total battles incremented to {}", config.total_battles);
        msg!("   🏆 Leaderboard: Rankings updated for both players");
        msg!("   ⚔️ Warriors: HP preserved (A: {}/100, B: {}/100)", 
            warrior_a.current_hp, warrior_b.current_hp);
        msg!("   ⏱️ Cooldowns: A expires {}, B expires {}", 
            warrior_a.cooldown_expires_at, warrior_b.cooldown_expires_at);

        Ok(())
    }
}