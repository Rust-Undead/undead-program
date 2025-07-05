use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BattleRoom {
    pub room_id: [u8; 32],                    // Unique room identifier
    pub created_at: i64,                      // Room creation timestamp
    pub player_a: Pubkey,                     // Room creator
    pub player_b: Option<Pubkey>,             // Room joiner (None until joined)
    pub warrior_a: Pubkey,                    // Player A's warrior
    pub warrior_b: Option<Pubkey>,            // Player B's warrior (None until joined)
    pub selected_concepts: [u8; 5],           // VRF-selected concept IDs [2, 5, 7, 9, 10]
    pub selected_topics : [u8; 10],
    pub selected_questions: [u16; 10],        // VRF-selected question IDs [23, 37,68, ...]
    pub correct_answers: [bool; 10],          // Correct answers for questions
    pub state: BattleState,                   // Current battle phase
    pub player_a_ready: bool,                 // Player A ready for battle
    pub player_b_ready: bool,                 // Player B ready for battle
    pub current_question: u8,                 // Current question index (0-9)
    pub player_a_answers: [Option<bool>; 10], // Player A's answers
    pub player_b_answers: [Option<bool>; 10], // Player B's answers
    pub player_a_correct: u8,                 // Player A's correct count
    pub player_b_correct: u8,                 // Player B's correct count
    pub winner: Option<Pubkey>,               // Battle winner
    pub battle_duration: u32,                 // Battle time in seconds
    pub bump: u8,  
    pub battle_start_time: i64,                    
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BattleState {
    Created,              // Room created, selecting concepts
    Joined,              // Player B joined, selecting questions
    QuestionsSelected,    // Questions ready, study phase
    ReadyForDelegation,   // Both ready, can delegate
    InProgress,          // Battle happening
    Completed,           // Battle finished
    Cancelled,           // Room cancelled
}

impl Space for BattleState {
    const INIT_SPACE: usize = 1;
}

// create fxns that can be used for constraints and validations
impl BattleRoom {
    // === PLAYER VALIDATION ===
    pub fn is_player_in_room(&self, player: &Pubkey) -> bool {
        self.player_a == *player || self.player_b == Some(*player)
    }
    
    pub fn get_opponent(&self, player: &Pubkey) -> Option<Pubkey> {
        if self.player_a == *player {
            self.player_b
        } else if self.player_b == Some(*player) {
            Some(self.player_a)
        } else {
            None
        }
    }

    pub fn is_battle_active(&self) -> bool {
        self.state == BattleState::InProgress
    }
    
    pub fn is_battle_complete(&self) -> bool {
        matches!(self.state, BattleState::Completed)
    }
    
    // === PROGRESS ===
    pub fn get_scores(&self) -> (u8, u8) {
        (self.player_a_correct, self.player_b_correct)
    }
    
    pub fn has_player_answered(&self, player: &Pubkey, question_idx: u8) -> bool {
        let idx = question_idx as usize;
        if idx >= 10 { return false; }
        
        if self.player_a == *player {
            self.player_a_answers[idx].is_some()
        } else if self.player_b == Some(*player) {
            self.player_b_answers[idx].is_some()
        } else {
            false
        }
    }
}