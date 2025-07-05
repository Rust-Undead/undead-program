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

	//img fields
	pub image_rarity: ImageRarity,
  pub image_index: u8,
  #[max_len(200)]
  pub image_uri: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageRarity {
	Common,
	Uncommon,
	Rare
}

impl Space for ImageRarity {
	const INIT_SPACE: usize = 1;
}

impl std::fmt::Display for ImageRarity {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>
	) -> std::fmt::Result{
		match self {
            ImageRarity::Common => write!(f, "common"),
            ImageRarity::Uncommon => write!(f, "uncommon"),
            ImageRarity::Rare => write!(f, "rare"),
        }
	}
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

impl std::fmt::Display for WarriorClass {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>
	) -> std::fmt::Result{
		match self {
						WarriorClass::Validator => write!(f, "Validator Warrior"),
						WarriorClass::Oracle => write!(f, "Oracle Warrior"),
						WarriorClass::Guardian => write!(f, "Guardian Warrior"),
						WarriorClass::Daemon => write!(f, "Daemon Warrior"),
        }
	}
 }
