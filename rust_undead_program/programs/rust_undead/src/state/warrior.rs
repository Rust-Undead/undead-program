use anchor_lang::prelude::*;

// warrior stats state definition. 
#[account]
#[derive(InitSpace)]
pub struct UndeadWarrior {
	#[max_len(32)]
  pub name: String,
	pub owner: Pubkey,
	pub dna: [u8; 8],
	pub created_at: i64,
	pub base_attack: u16,
	pub base_defense: u16,
	pub base_knowledge: u16,
	pub current_hp: u16,
  pub max_hp: u16,                
	pub warrior_class: WarriorClass,
	pub battles_won: u32,
	pub battles_lost: u32,
	pub experience_points: u64,
	pub level: u16,
	pub last_battle_at: i64,  
  pub cooldown_expires_at: i64,
	pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarriorClass {
	Validator,
	Oracle,
	Guardian,
	Daemon
}

impl Space for WarriorClass {
	const INIT_SPACE : usize = 1;
}
