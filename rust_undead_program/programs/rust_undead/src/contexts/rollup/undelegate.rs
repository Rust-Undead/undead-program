use anchor_lang::prelude::*;
use crate::state::*;
use crate::constants::*;
use crate::error::RustUndeadError;

use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

//undelegate all we delegated before 
#[commit]
#[derive(Accounts)]
#[instruction(room_id: [u8; 32])]
pub struct UndelegateBattleRoom<'info> {
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



impl<'info> UndelegateBattleRoom<'info> {
    pub fn undelegate_battle_room(
        &mut self,
        room_id: [u8; 32],
    ) -> Result<()> {

      require!(room_id == self.battle_room.room_id, RustUndeadError::InvalidRoomId);

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

        Ok(())
    }
}