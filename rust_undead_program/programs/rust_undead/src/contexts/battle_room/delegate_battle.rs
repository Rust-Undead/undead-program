use anchor_lang::prelude::*;
use crate::constants::*;
//MB
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;

#[delegate]
#[derive(Accounts)]
#[instruction(room_id: [u8; 32], player_a: Pubkey, warrior_a_name: String, player_b: Pubkey, warrior_b_name: String)]
pub struct DelegateBattle<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: The Battle Room account we are delegating
    #[account(
        mut,
        del,
        seeds = [BATTLE, room_id.as_ref()],
        bump,
    )]
    pub battle_room: AccountInfo<'info>,

    /// CHECK: Warrior A account we are delegating
    #[account(
        mut,
        del,
        seeds = [UNDEAD_WARRIOR, player_a.as_ref(), warrior_a_name.as_bytes()],
        bump,
    )]
    pub warrior_a: AccountInfo<'info>,

    /// CHECK: Warrior B account we are delegating
    #[account(
        mut,
        del,
        seeds = [UNDEAD_WARRIOR, player_b.as_ref(), warrior_b_name.as_bytes()],
        bump,
    )]
    pub warrior_b: AccountInfo<'info>,
}

impl<'info> DelegateBattle<'info> {
    pub fn delegate_to_rollup(
        &mut self,
        room_id: [u8; 32],
        player_a: Pubkey,
        warrior_a_name: String,
        player_b: Pubkey,
        warrior_b_name: String,
    ) -> Result<()> {
        msg!("Delegating battle room and warriors to ephemeral rollup...");
        // Delegate battle room
        
        self.delegate_battle_room(
            &self.authority,
            &[BATTLE, room_id.as_ref()],
            DelegateConfig::default(),
        )?;
        
        // Delegate warrior A
        self.delegate_warrior_a(
            &self.authority,
            &[UNDEAD_WARRIOR, player_a.as_ref(), warrior_a_name.as_bytes()],
            DelegateConfig::default(),
        )?;
        
        // Delegate warrior B
        self.delegate_warrior_b(
            &self.authority,
            &[UNDEAD_WARRIOR, player_b.as_ref(), warrior_b_name.as_bytes()],
            DelegateConfig::default(),
        )?;
        
        msg!(
            "Successfully delegated battle room {} and warriors to ephemeral rollup"
        );
        
        msg!(
            "Delegated warriors: {} (Player A) vs {} (Player B)",
            warrior_a_name,
            warrior_b_name
        );
        
        Ok(())
    }
}