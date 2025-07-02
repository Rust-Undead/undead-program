use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;

use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;

//undelegate all we delegated before 
#[commit]
#[derive(Accounts)]
#[instruction(room_id: [u8; 32])]
pub struct EndBattleRoom<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
        constraint = battle_room.room_id == room_id @ RustUndeadError::InvalidRoomId,
        constraint = battle_room.state == BattleState::Completed @ RustUndeadError::InvalidBattleState,
        constraint = battle_room.winner.is_some() @ RustUndeadError::CannotUndelegate,
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
}



impl<'info> EndBattleRoom<'info> {
    pub fn end_battle_room(
        &mut self,
        room_id: [u8; 32],
    ) -> Result<()> {
        let battle_room = &mut self.battle_room;
        let warrior_a = &mut self.warrior_a;
        let warrior_b = &mut self.warrior_b;

        // Validate battle is complete and has winner
        require!(
            battle_room.state == BattleState::Completed,
            RustUndeadError::InvalidBattleState
        );

        // Determine winner and loser
        let winner_player = battle_room.winner.unwrap();
        let is_player_a_winner = winner_player == battle_room.player_a;

        let (winner_warrior, loser_warrior) = if is_player_a_winner {
            (warrior_a, warrior_b)
        } else {
            (warrior_b, warrior_a)
        };

      
        msg!("🏆 Settling battle results...");
        msg!("   Room ID: {:?}", room_id);
        msg!("   Winner: {} ({})", 
            if is_player_a_winner { "Player A" } else { "Player B" },
            winner_player
        );
        msg!("   Battle Duration: {} seconds", battle_room.battle_duration);

        // ✅ ALL DATA MODIFICATIONS BEFORE COMMIT
        // Update Warrior Battle Records
        winner_warrior.battles_won = winner_warrior.battles_won.saturating_add(1);
        loser_warrior.battles_lost = loser_warrior.battles_lost.saturating_add(1);

        // Calculate Simple Experience Rewards
        let base_xp_winner = 40; 
        let base_xp_loser = 20; 
        
        let winner_score = if is_player_a_winner { 
            battle_room.player_a_correct 
        } else { 
            battle_room.player_b_correct 
        };
        
        let loser_score = battle_room.player_a_correct + battle_room.player_b_correct - winner_score;
        
        // Simple XP calculation
        let score_bonus = (winner_score as u64) * 4;  // 4 XP per correct answer
        let winner_xp = base_xp_winner + score_bonus;
        let loser_xp = base_xp_loser + (loser_score as u64 * 2); // 2 XP per correct answer

        winner_warrior.experience_points = winner_warrior.experience_points.saturating_add(winner_xp);
        loser_warrior.experience_points = loser_warrior.experience_points.saturating_add(loser_xp);

       
        msg!("💎 Experience Awarded:");
        msg!("   {} gained {} XP (Base: {} + Score: {})", 
            winner_warrior.name, winner_xp, base_xp_winner, score_bonus);
        msg!("   {} gained {} XP (Base: {} + Score: {})", 
            loser_warrior.name, loser_xp, base_xp_loser, loser_score as u64 * 2);

        msg!("📊 Final Battle Statistics:");
        msg!("   Questions Answered - A: {}, B: {}", battle_room.player_a_correct, battle_room.player_b_correct);
        msg!("   Final HP - {}: {}, {}: {}", 
            winner_warrior.name, winner_warrior.current_hp,
            loser_warrior.name, loser_warrior.current_hp
        );
        msg!("   Victory Type: {}", 
            if loser_warrior.current_hp == 0 { "Elimination" } else { "HP/Score Decision" }
        );
        msg!("   XP Distribution - Winner: {}, Loser: {}", winner_xp, loser_xp);

        
        msg!("🏁 Battle settlement complete! All delegated accounts will be undelegated from rollup");
        msg!("✅ Warriors and battle room will be returned to mainnet with updated stats");
        msg!("🎯 Ready for next battles after cooldown period");

        
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

        Ok(())
    }
}