use anchor_lang::prelude::*;

#[error_code]
pub enum RustUndeadError {
	#[msg("Warrior is not in the correct state for this action")]
	InvalidWarriorState,
	#[msg("Only the warrior owner can perform this action")]
	NotWarriorOwner,
	#[msg("Warrior name exceeds maximum length")]
	WarriorNameTooLong,
	#[msg("Warrior is already in a battle")]
	WarriorAlreadyInBattle,
	#[msg("Warrior Name already exists")]
	WarriorAlreadyExists,
	#[msg("Warrior On Cooldown")]
	WarriorOnCooldown,
	#[msg("Battle room is not in the correct state for this action")]
	InvalidBattleState,
	#[msg("Battle room already has two players")]
	BattleRoomFull,
	#[msg("Only battle participants can perform this action")]
	NotBattleParticipant,
	#[msg("Player has not marked themselves as ready")]
	PlayerNotReady,
	#[msg("Battle results have already been settled")]
	BattleAlreadySettled,
	#[msg("Room ID is invalid or too long")]
	InvalidRoomId,
	#[msg("VRF request is still pending")]
	VrfRequestPending,
	#[msg("Invalid VRF result received")]
	InvalidVrfResult,
	#[msg("VRF request not found or expired")]
	VrfRequestNotFound,
	#[msg("Not authorized to perform this action")]
	NotAuthorized,
	#[msg("Invalid settlement authority")]
	InvalidSettlementAuthority,
	#[msg("Insufficient funds for this operation")]
	InsufficientFunds,
	#[msg("Invalid concept selection")]
	InvalidConceptSelection,
	#[msg("Game state has not been initialized")]
	GameNotInitialized,
	#[msg("Invalid ephemeral rollup session ID")]
	InvalidErSessionId,
	#[msg("Player not in room")]
	PlayerNotInRoom,
	#[msg("Player is ready")]
	AlreadyReady,
	#[msg("Invalid Warrior")]
	InvalidWarrior,
	#[msg("Same Warrior cannot Battle")]
	SameWarriorCannotBattle,
	#[msg("Warrior defeated")]
	WarriorDefeated	,
	#[msg("Player has already answered this question")]
	AlreadyAnswered,
	#[msg("All Questions answered")]
	AllQuestionsAnswered,
	#[msg("Name is too long, consider reducing it")]
	NameTooLong,
	#[msg("Invalid, please input name")]
	NameEmpty,
	#[msg("Warrior cannot attack itself")]
	CannotAttackSelf,
	#[msg("Invalid Question Index")]
	 InvalidQuestionIndex,
	#[msg("Only the room creator can cancel the battle")]
  OnlyCreatorCanCancel,
	#[msg("Battle has already started and cannot be cancelled")]
  BattleAlreadyStarted,
  #[msg("Battle has already been completed")]
  BattleAlreadyCompleted,
  #[msg("Battle room has already been cancelled")]
    BattleAlreadyCancelled ,
  #[msg("Cannot cancel battle at this stage")]
    CannotCancelAtThisStage,
	#[msg("Game not ready for Undelegation")]
    CannotUndelegate,
	#[msg("Invalid image index for the selected rarity")]
    InvalidImageIndex,
  #[msg("Invalid warrior class and rarity combination")]
    InvalidClassRarity,
  #[msg("Image generation failed")]
    ImageGenerationFailed,
}
