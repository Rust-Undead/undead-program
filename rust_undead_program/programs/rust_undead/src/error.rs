// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.
//
// Docs: https://docs.codigo.ai/c%C3%B3digo-interface-description-language/specification#errors

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
}
