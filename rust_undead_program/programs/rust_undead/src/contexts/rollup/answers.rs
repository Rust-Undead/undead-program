use anchor_lang::prelude::*;
use crate::helpers::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_accounts;

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
}

impl<'info> AnswerQuestion<'info> {
    pub fn answer_question(
        &mut self,
        room_id: [u8; 32],
        answer: bool,
        client_seed: u8,
    ) -> Result<()> {
        // Validate room ID
        require!(self.battle_room.room_id == room_id, RustUndeadError::InvalidRoomId);

        let player = self.player.key();
        let current_q = self.battle_room.current_question as usize;

        require!(current_q < 10, RustUndeadError::InvalidQuestionIndex);
        require!(!self.battle_room.has_player_answered(&player, self.battle_room.current_question),
                RustUndeadError::AlreadyAnswered);
        
        // Record player's answer privately
        if self.battle_room.player_a == player {
            self.battle_room.player_a_answers[current_q] = Some(answer);
            msg!("🎯 Player A submitted answer for question {}", current_q + 1);
        } else if self.battle_room.player_b == Some(player) {
            self.battle_room.player_b_answers[current_q] = Some(answer);
            msg!("🎯 Player B submitted answer for question {}", current_q + 1);
        } else {
            return Err(RustUndeadError::PlayerNotInRoom.into());
        }
        
        // ✅ Check if both players have answered
        let both_answered = self.battle_room.player_a_answers[current_q].is_some() 
            && self.battle_room.player_b_answers[current_q].is_some();
        
        if !both_answered {
            // Only one player has answered - wait for opponent
            msg!("⏳ Answer submitted! Waiting for opponent to answer question {}", current_q + 1);
            return Ok(());
        }

        // ✅ Both players answered - reveal and process results
        msg!("🎭 Both players submitted! Revealing answers...");
        
        let player_a_answer = self.battle_room.player_a_answers[current_q].unwrap();
        let player_b_answer = self.battle_room.player_b_answers[current_q].unwrap();
        let correct_answer = self.battle_room.correct_answers[current_q];
        
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
        
        // ✅ Process answers and update scores
        let player_a_correct = player_a_answer == correct_answer;
        let player_b_correct = player_b_answer == correct_answer;
        
        if player_a_correct {
            self.battle_room.player_a_correct += 1;
            msg!("⚔️ Player A gets point!");
        }
        
        if player_b_correct {
            self.battle_room.player_b_correct += 1;
            msg!("⚔️ Player B gets point!");
        }
        
        msg!("📊 Updated Scores - A: {}, B: {}", 
            self.battle_room.player_a_correct, self.battle_room.player_b_correct);
        

        // ✅ Handle DETERMINISTIC damage calculation for each correct answer
        if player_a_correct {
            msg!("🗡️ Player A correct - calculating damage to Player B's warrior...");

            let (attacking_warrior, defending_warrior, attacker_key, defender_key) = if self.battle_room.player_a == self.player.key() {
                // current player is player A who is the attacker (Caller of the fxn)
                (&mut self.attacker_warrior, &mut self.defender_warrior, self.battle_room.warrior_a, self.battle_room.warrior_b.unwrap())
            } else {
                (&mut self.defender_warrior, &mut self.attacker_warrior, self.battle_room.warrior_a, self.battle_room.warrior_b.unwrap())
            };
            
            // Calculate damage using helper function
            let final_damage = calculate_deterministic_damage_with_keys(
                attacking_warrior,
                defending_warrior,
                attacker_key,
                defender_key,
                current_q,
                room_id,
                client_seed,
            )?;

            // Apply damage to defender (Player B's warrior)
            let old_hp = defending_warrior.current_hp;
            defending_warrior.current_hp = defending_warrior.current_hp.saturating_sub(final_damage);
            let new_hp = defending_warrior.current_hp;

            msg!("🩸 {} takes {} damage! HP: {} → {}", 
                defending_warrior.name, final_damage, old_hp, new_hp);

            // Check for elimination
            if defending_warrior.current_hp == 0 {
                self.battle_room.winner = Some(attacking_warrior.owner);
                self.battle_room.state = BattleState::Completed;
                msg!("💀 {} has been defeated!", defending_warrior.name);
                msg!("🏆 {} wins by elimination!", attacking_warrior.name);
            } else if defending_warrior.current_hp <= 10 {
                msg!("⚠️ {} is critically wounded! ({} HP remaining)", 
                    defending_warrior.name, defending_warrior.current_hp);
            }
        }

        if player_b_correct && self.battle_room.state != BattleState::Completed {
            msg!("🗡️ Player B correct - calculating damage to Player A's warrior...");
            
            let (attacking_warrior, defending_warrior, attacker_key, defender_key) = if self.battle_room.player_b == Some(self.player.key()) {
                // current player is player B who is the attacker
                (&mut self.attacker_warrior, &mut self.defender_warrior, self.battle_room.warrior_b.unwrap(), self.battle_room.warrior_a)
            } else {
                (&mut self.defender_warrior, &mut self.attacker_warrior, self.battle_room.warrior_b.unwrap(), self.battle_room.warrior_a)
            };

            // Calculate damage using helper function (with different seed for Player B)
            let final_damage = calculate_deterministic_damage_with_keys(
                attacking_warrior,
                defending_warrior,
                attacker_key,
                defender_key,
                current_q,
                room_id,
                client_seed.wrapping_add(1), // Different seed for Player B
            )?;

            // Apply damage to defender (Player A's warrior)
            let old_hp = defending_warrior.current_hp;
            defending_warrior.current_hp = defending_warrior.current_hp.saturating_sub(final_damage);
            let new_hp = defending_warrior.current_hp;

            msg!("🩸 {} takes {} damage! HP: {} → {}", 
                defending_warrior.name, final_damage, old_hp, new_hp);

            // Check for elimination
            if defending_warrior.current_hp == 0 {
                self.battle_room.winner = Some(attacking_warrior.owner);
                self.battle_room.state = BattleState::Completed;
                msg!("💀 {} has been defeated!", defending_warrior.name);
                msg!("🏆 {} wins by elimination!", attacking_warrior.name);
            } else if defending_warrior.current_hp <= 10 {
                msg!("⚠️ {} is critically wounded! ({} HP remaining)", 
                    defending_warrior.name, defending_warrior.current_hp);
            }
        }
        
        if !player_a_correct && !player_b_correct {
            msg!("❌ Neither player correct - no damage dealt, moving to next question");
        }
        
        // ✅ STEP 6: Advance question or end battle
        if current_q < 9 && self.battle_room.state != BattleState::Completed {
            self.battle_room.current_question += 1;
            msg!("📋 Moving to question {}", self.battle_room.current_question + 1);
        } else if current_q == 9 && self.battle_room.state != BattleState::Completed {
            msg!("🏁 All questions completed! Determining final winner by HP...");
            
            let warrior_a_hp = if self.battle_room.warrior_a == self.attacker_warrior.key() {
                self.attacker_warrior.current_hp
            } else {
                self.defender_warrior.current_hp
            };
            let warrior_b_hp = if self.battle_room.warrior_b.unwrap() == self.attacker_warrior.key() {
                self.attacker_warrior.current_hp
            } else {
                self.defender_warrior.current_hp
            };
            
            if warrior_a_hp > warrior_b_hp {
                self.battle_room.winner = Some(self.battle_room.player_a);
                msg!("🏆 Player A wins with {} HP vs {} HP!", warrior_a_hp, warrior_b_hp);
            } else if warrior_b_hp > warrior_a_hp {
                self.battle_room.winner = Some(self.battle_room.player_b.unwrap());
                msg!("🏆 Player B wins with {} HP vs {} HP!", warrior_b_hp, warrior_a_hp);
            } else {
                let (score_a, score_b) = self.battle_room.get_scores();
                if score_a > score_b {
                    self.battle_room.winner = Some(self.battle_room.player_a);
                    msg!("🏆 HP tied at {}! Player A wins by score: {} vs {}", warrior_a_hp, score_a, score_b);
                } else if score_b > score_a {
                    self.battle_room.winner = Some(self.battle_room.player_b.unwrap());
                    msg!("🏆 HP tied at {}! Player B wins by score: {} vs {}", warrior_b_hp, score_b, score_a);
                } else {
                    self.battle_room.winner = Some(self.battle_room.player_a);
                    msg!("🏆 Perfect tie! HP: {}, Score: {} each - Player A wins by default", warrior_a_hp, score_a);
                }
            }
            self.battle_room.state = BattleState::Completed;
        }
        
        // ✅ STEP 7: Update battle timing
        let current_time = Clock::get()?.unix_timestamp;
        self.battle_room.battle_duration = (current_time - self.battle_room.battle_start_time) as u32;

        // ✅ STEP 8: Only commit when battle is complete
        if self.battle_room.state == BattleState::Completed {
            commit_accounts(
                &self.player,
                vec![
                    &self.battle_room.to_account_info(),
                    &self.attacker_warrior.to_account_info(),
                    &self.defender_warrior.to_account_info(),
                ],
                &self.magic_context,  
                &self.magic_program,
            )?;
            msg!("🏁 Battle completed! Final state committed to rollup for settlement");
        } else {
            msg!("⚔️ Battle continues... Question {} ready", self.battle_room.current_question + 1);
        }
       
        Ok(())
    }
}