import {
  AnchorProvider,
  BN,
  IdlAccounts,
  Program,
  web3,
} from "@coral-xyz/anchor";
import { MethodsBuilder } from "@coral-xyz/anchor/dist/cjs/program/namespace/methods";
import { RustUndead } from "../../target/types/rust_undead";
import idl from "../../target/idl/rust_undead.json";
import * as pda from "./pda";



let _program: Program<RustUndead>;


export const initializeClient = (
    programId: web3.PublicKey,
    anchorProvider = AnchorProvider.env(),
) => {
    _program = new Program<RustUndead>(
        idl as never,
        programId,
        anchorProvider,
    );


};

export type InitializeGameArgs = {
  feePayer: web3.PublicKey;
  authority: web3.PublicKey;
  systemProgram: web3.PublicKey;
  vrfOracle: web3.PublicKey;
  erBridgeAuthority: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Initialize the global game state
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} The game authority
 * 2. `[writable]` game_state: {@link GameState} The global game state account
 * 3. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - vrf_oracle: {@link PublicKey} Magic Block VRF oracle pubkey
 * - er_bridge_authority: {@link PublicKey} Ephemeral rollup bridge authority
 */
export const initializeGameBuilder = (
	args: InitializeGameArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {
  const [gameStatePubkey] = pda.deriveGameStateSeedPDA(_program.programId);

  return _program
    .methods
    .initializeGame(
      args.vrfOracle,
      args.erBridgeAuthority,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      authority: args.authority,
      gameState: gameStatePubkey,
      systemProgram: args.systemProgram,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Initialize the global game state
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} The game authority
 * 2. `[writable]` game_state: {@link GameState} The global game state account
 * 3. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - vrf_oracle: {@link PublicKey} Magic Block VRF oracle pubkey
 * - er_bridge_authority: {@link PublicKey} Ephemeral rollup bridge authority
 */
export const initializeGame = (
	args: InitializeGameArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    initializeGameBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Initialize the global game state
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} The game authority
 * 2. `[writable]` game_state: {@link GameState} The global game state account
 * 3. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - vrf_oracle: {@link PublicKey} Magic Block VRF oracle pubkey
 * - er_bridge_authority: {@link PublicKey} Ephemeral rollup bridge authority
 */
export const initializeGameSendAndConfirm = async (
  args: Omit<InitializeGameArgs, "feePayer" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return initializeGameBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.authority])
    .rpc();
}

export type CreateWarriorArgs = {
  feePayer: web3.PublicKey;
  owner: web3.PublicKey;
  vrfClient: web3.PublicKey;
  vrfProgram: web3.PublicKey;
  systemProgram: web3.PublicKey;
  name: string;
  dna: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Create a new warrior with DNA and request VRF for stats
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` owner: {@link PublicKey} The owner of the warrior
 * 2. `[writable]` warrior: {@link Warrior} The warrior account
 * 3. `[writable]` game_state: {@link GameState} The global game state account
 * 4. `[writable]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 5. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 * 6. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - name: {@link string} The name of the warrior
 * - dna: {@link BigInt} DNA for visual generation
 */
export const createWarriorBuilder = (
	args: CreateWarriorArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {
    const [warriorPubkey] = pda.deriveWarriorSeedPDA({
        owner: args.owner,
        name: args.name,
    }, _program.programId);
  const [gameStatePubkey] = pda.deriveGameStateSeedPDA(_program.programId);

  return _program
    .methods
    .createWarrior(
      args.name,
      new BN(args.dna.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      owner: args.owner,
      warrior: warriorPubkey,
      gameState: gameStatePubkey,
      vrfClient: args.vrfClient,
      vrfProgram: args.vrfProgram,
      systemProgram: args.systemProgram,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Create a new warrior with DNA and request VRF for stats
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` owner: {@link PublicKey} The owner of the warrior
 * 2. `[writable]` warrior: {@link Warrior} The warrior account
 * 3. `[writable]` game_state: {@link GameState} The global game state account
 * 4. `[writable]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 5. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 * 6. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - name: {@link string} The name of the warrior
 * - dna: {@link BigInt} DNA for visual generation
 */
export const createWarrior = (
	args: CreateWarriorArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createWarriorBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Create a new warrior with DNA and request VRF for stats
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` owner: {@link PublicKey} The owner of the warrior
 * 2. `[writable]` warrior: {@link Warrior} The warrior account
 * 3. `[writable]` game_state: {@link GameState} The global game state account
 * 4. `[writable]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 5. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 * 6. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - name: {@link string} The name of the warrior
 * - dna: {@link BigInt} DNA for visual generation
 */
export const createWarriorSendAndConfirm = async (
  args: Omit<CreateWarriorArgs, "feePayer" | "owner"> & {
    signers: {
      feePayer: web3.Signer,
      owner: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createWarriorBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      owner: args.signers.owner.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.owner])
    .rpc();
}

export type FinalizeWarriorStatsArgs = {
  feePayer: web3.PublicKey;
  warrior: web3.PublicKey;
  owner: web3.PublicKey;
  vrfClient: web3.PublicKey;
  vrfProgram: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Finalize warrior creation with VRF randomness result
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` warrior: {@link Warrior} The warrior account
 * 2. `[]` owner: {@link PublicKey} The warrior owner (for verification)
 * 3. `[]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 4. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const finalizeWarriorStatsBuilder = (
	args: FinalizeWarriorStatsArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {


  return _program
    .methods
    .finalizeWarriorStats(

    )
    .accountsStrict({
      feePayer: args.feePayer,
      warrior: args.warrior,
      owner: args.owner,
      vrfClient: args.vrfClient,
      vrfProgram: args.vrfProgram,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Finalize warrior creation with VRF randomness result
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` warrior: {@link Warrior} The warrior account
 * 2. `[]` owner: {@link PublicKey} The warrior owner (for verification)
 * 3. `[]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 4. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const finalizeWarriorStats = (
	args: FinalizeWarriorStatsArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    finalizeWarriorStatsBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Finalize warrior creation with VRF randomness result
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` warrior: {@link Warrior} The warrior account
 * 2. `[]` owner: {@link PublicKey} The warrior owner (for verification)
 * 3. `[]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 4. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const finalizeWarriorStatsSendAndConfirm = async (
  args: Omit<FinalizeWarriorStatsArgs, "feePayer"> & {
    signers: {
      feePayer: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return finalizeWarriorStatsBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer])
    .rpc();
}

export type CreateBattleRoomArgs = {
  feePayer: web3.PublicKey;
  creator: web3.PublicKey;
  warrior: web3.PublicKey;
  systemProgram: web3.PublicKey;
  roomId: string;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Create a new battle room
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` creator: {@link PublicKey} The creator of the battle room
 * 2. `[writable]` warrior: {@link Warrior} The creator's warrior
 * 3. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 4. `[writable]` game_state: {@link GameState} The global game state account
 * 5. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - room_id: {@link string} Unique identifier for the battle room
 */
export const createBattleRoomBuilder = (
	args: CreateBattleRoomArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {
    const [battleRoomPubkey] = pda.deriveBattleRoomSeedPDA({
        roomId: args.roomId,
    }, _program.programId);
  const [gameStatePubkey] = pda.deriveGameStateSeedPDA(_program.programId);

  return _program
    .methods
    .createBattleRoom(
      args.roomId,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      creator: args.creator,
      warrior: args.warrior,
      battleRoom: battleRoomPubkey,
      gameState: gameStatePubkey,
      systemProgram: args.systemProgram,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Create a new battle room
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` creator: {@link PublicKey} The creator of the battle room
 * 2. `[writable]` warrior: {@link Warrior} The creator's warrior
 * 3. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 4. `[writable]` game_state: {@link GameState} The global game state account
 * 5. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - room_id: {@link string} Unique identifier for the battle room
 */
export const createBattleRoom = (
	args: CreateBattleRoomArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    createBattleRoomBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Create a new battle room
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` creator: {@link PublicKey} The creator of the battle room
 * 2. `[writable]` warrior: {@link Warrior} The creator's warrior
 * 3. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 4. `[writable]` game_state: {@link GameState} The global game state account
 * 5. `[]` system_program: {@link PublicKey} The system program
 *
 * Data:
 * - room_id: {@link string} Unique identifier for the battle room
 */
export const createBattleRoomSendAndConfirm = async (
  args: Omit<CreateBattleRoomArgs, "feePayer" | "creator"> & {
    signers: {
      feePayer: web3.Signer,
      creator: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return createBattleRoomBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      creator: args.signers.creator.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.creator])
    .rpc();
}

export type JoinBattleRoomArgs = {
  feePayer: web3.PublicKey;
  joiner: web3.PublicKey;
  warrior: web3.PublicKey;
  battleRoom: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Join an existing battle room
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` joiner: {@link PublicKey} The player joining the battle
 * 2. `[writable]` warrior: {@link Warrior} The joiner's warrior
 * 3. `[writable]` battle_room: {@link BattleRoom} The battle room account
 */
export const joinBattleRoomBuilder = (
	args: JoinBattleRoomArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {


  return _program
    .methods
    .joinBattleRoom(

    )
    .accountsStrict({
      feePayer: args.feePayer,
      joiner: args.joiner,
      warrior: args.warrior,
      battleRoom: args.battleRoom,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Join an existing battle room
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` joiner: {@link PublicKey} The player joining the battle
 * 2. `[writable]` warrior: {@link Warrior} The joiner's warrior
 * 3. `[writable]` battle_room: {@link BattleRoom} The battle room account
 */
export const joinBattleRoom = (
	args: JoinBattleRoomArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    joinBattleRoomBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Join an existing battle room
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` joiner: {@link PublicKey} The player joining the battle
 * 2. `[writable]` warrior: {@link Warrior} The joiner's warrior
 * 3. `[writable]` battle_room: {@link BattleRoom} The battle room account
 */
export const joinBattleRoomSendAndConfirm = async (
  args: Omit<JoinBattleRoomArgs, "feePayer" | "joiner"> & {
    signers: {
      feePayer: web3.Signer,
      joiner: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return joinBattleRoomBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      joiner: args.signers.joiner.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.joiner])
    .rpc();
}

export type SelectBattleConceptsArgs = {
  feePayer: web3.PublicKey;
  authority: web3.PublicKey;
  battleRoom: web3.PublicKey;
  vrfClient: web3.PublicKey;
  vrfProgram: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Use VRF to select 5 concepts for the battle
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} Either player can trigger concept selection
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 4. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const selectBattleConceptsBuilder = (
	args: SelectBattleConceptsArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {


  return _program
    .methods
    .selectBattleConcepts(

    )
    .accountsStrict({
      feePayer: args.feePayer,
      authority: args.authority,
      battleRoom: args.battleRoom,
      vrfClient: args.vrfClient,
      vrfProgram: args.vrfProgram,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Use VRF to select 5 concepts for the battle
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} Either player can trigger concept selection
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 4. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const selectBattleConcepts = (
	args: SelectBattleConceptsArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    selectBattleConceptsBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Use VRF to select 5 concepts for the battle
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} Either player can trigger concept selection
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 4. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const selectBattleConceptsSendAndConfirm = async (
  args: Omit<SelectBattleConceptsArgs, "feePayer" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return selectBattleConceptsBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.authority])
    .rpc();
}

export type FinalizeConceptSelectionArgs = {
  feePayer: web3.PublicKey;
  battleRoom: web3.PublicKey;
  vrfClient: web3.PublicKey;
  vrfProgram: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Finalize concept selection with VRF result
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 2. `[]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 3. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const finalizeConceptSelectionBuilder = (
	args: FinalizeConceptSelectionArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {


  return _program
    .methods
    .finalizeConceptSelection(

    )
    .accountsStrict({
      feePayer: args.feePayer,
      battleRoom: args.battleRoom,
      vrfClient: args.vrfClient,
      vrfProgram: args.vrfProgram,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Finalize concept selection with VRF result
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 2. `[]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 3. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const finalizeConceptSelection = (
	args: FinalizeConceptSelectionArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    finalizeConceptSelectionBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Finalize concept selection with VRF result
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} Auto-generated, default fee payer
 * 1. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 2. `[]` vrf_client: {@link PublicKey} The Magic Block VRF client account
 * 3. `[]` vrf_program: {@link PublicKey} The Magic Block VRF program
 */
export const finalizeConceptSelectionSendAndConfirm = async (
  args: Omit<FinalizeConceptSelectionArgs, "feePayer"> & {
    signers: {
      feePayer: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return finalizeConceptSelectionBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer])
    .rpc();
}

export type MarkReadyForBattleArgs = {
  feePayer: web3.PublicKey;
  player: web3.PublicKey;
  battleRoom: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Mark a player as ready for battle after studying concepts
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` player: {@link PublicKey} The player marking themselves as ready
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 */
export const markReadyForBattleBuilder = (
	args: MarkReadyForBattleArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {


  return _program
    .methods
    .markReadyForBattle(

    )
    .accountsStrict({
      feePayer: args.feePayer,
      player: args.player,
      battleRoom: args.battleRoom,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Mark a player as ready for battle after studying concepts
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` player: {@link PublicKey} The player marking themselves as ready
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 */
export const markReadyForBattle = (
	args: MarkReadyForBattleArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    markReadyForBattleBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Mark a player as ready for battle after studying concepts
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` player: {@link PublicKey} The player marking themselves as ready
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 */
export const markReadyForBattleSendAndConfirm = async (
  args: Omit<MarkReadyForBattleArgs, "feePayer" | "player"> & {
    signers: {
      feePayer: web3.Signer,
      player: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return markReadyForBattleBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      player: args.signers.player.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.player])
    .rpc();
}

export type DelegateToEphemeralRollupArgs = {
  feePayer: web3.PublicKey;
  authority: web3.PublicKey;
  battleRoom: web3.PublicKey;
  warriorA: web3.PublicKey;
  warriorB: web3.PublicKey;
  erSessionId: bigint;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Delegate warriors to ephemeral rollup for real-time battle
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} Either player can trigger delegation
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` warrior_a: {@link Warrior} First warrior in the battle
 * 4. `[writable]` warrior_b: {@link Warrior} Second warrior in the battle
 *
 * Data:
 * - er_session_id: {@link BigInt} Ephemeral rollup session identifier
 */
export const delegateToEphemeralRollupBuilder = (
	args: DelegateToEphemeralRollupArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {


  return _program
    .methods
    .delegateToEphemeralRollup(
      new BN(args.erSessionId.toString()),
    )
    .accountsStrict({
      feePayer: args.feePayer,
      authority: args.authority,
      battleRoom: args.battleRoom,
      warriorA: args.warriorA,
      warriorB: args.warriorB,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Delegate warriors to ephemeral rollup for real-time battle
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} Either player can trigger delegation
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` warrior_a: {@link Warrior} First warrior in the battle
 * 4. `[writable]` warrior_b: {@link Warrior} Second warrior in the battle
 *
 * Data:
 * - er_session_id: {@link BigInt} Ephemeral rollup session identifier
 */
export const delegateToEphemeralRollup = (
	args: DelegateToEphemeralRollupArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    delegateToEphemeralRollupBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Delegate warriors to ephemeral rollup for real-time battle
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} Either player can trigger delegation
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` warrior_a: {@link Warrior} First warrior in the battle
 * 4. `[writable]` warrior_b: {@link Warrior} Second warrior in the battle
 *
 * Data:
 * - er_session_id: {@link BigInt} Ephemeral rollup session identifier
 */
export const delegateToEphemeralRollupSendAndConfirm = async (
  args: Omit<DelegateToEphemeralRollupArgs, "feePayer" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return delegateToEphemeralRollupBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.authority])
    .rpc();
}

export type SettleBattleResultsArgs = {
  feePayer: web3.PublicKey;
  settlementAuthority: web3.PublicKey;
  battleRoom: web3.PublicKey;
  warriorA: web3.PublicKey;
  warriorB: web3.PublicKey;
  winner: web3.PublicKey;
  totalQuestions: number;
  playerACorrect: number;
  playerBCorrect: number;
  battleDuration: number;
  criticalHits: number;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Settle battle results from ephemeral rollup back to mainnet
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` settlement_authority: {@link PublicKey} Authorized settlement bridge or admin
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` warrior_a: {@link Warrior} First warrior in the battle
 * 4. `[writable]` warrior_b: {@link Warrior} Second warrior in the battle
 * 5. `[writable]` game_state: {@link GameState} The global game state account
 *
 * Data:
 * - winner: {@link PublicKey} The winner of the battle
 * - total_questions: {@link number} Total questions in the battle
 * - player_a_correct: {@link number} Correct answers by player A
 * - player_b_correct: {@link number} Correct answers by player B
 * - battle_duration: {@link number} Battle duration in seconds
 * - critical_hits: {@link number} Total critical hits
 */
export const settleBattleResultsBuilder = (
	args: SettleBattleResultsArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {
  const [gameStatePubkey] = pda.deriveGameStateSeedPDA(_program.programId);

  return _program
    .methods
    .settleBattleResults(
      args.winner,
      args.totalQuestions,
      args.playerACorrect,
      args.playerBCorrect,
      args.battleDuration,
      args.criticalHits,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      settlementAuthority: args.settlementAuthority,
      battleRoom: args.battleRoom,
      warriorA: args.warriorA,
      warriorB: args.warriorB,
      gameState: gameStatePubkey,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Settle battle results from ephemeral rollup back to mainnet
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` settlement_authority: {@link PublicKey} Authorized settlement bridge or admin
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` warrior_a: {@link Warrior} First warrior in the battle
 * 4. `[writable]` warrior_b: {@link Warrior} Second warrior in the battle
 * 5. `[writable]` game_state: {@link GameState} The global game state account
 *
 * Data:
 * - winner: {@link PublicKey} The winner of the battle
 * - total_questions: {@link number} Total questions in the battle
 * - player_a_correct: {@link number} Correct answers by player A
 * - player_b_correct: {@link number} Correct answers by player B
 * - battle_duration: {@link number} Battle duration in seconds
 * - critical_hits: {@link number} Total critical hits
 */
export const settleBattleResults = (
	args: SettleBattleResultsArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    settleBattleResultsBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Settle battle results from ephemeral rollup back to mainnet
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` settlement_authority: {@link PublicKey} Authorized settlement bridge or admin
 * 2. `[writable]` battle_room: {@link BattleRoom} The battle room account
 * 3. `[writable]` warrior_a: {@link Warrior} First warrior in the battle
 * 4. `[writable]` warrior_b: {@link Warrior} Second warrior in the battle
 * 5. `[writable]` game_state: {@link GameState} The global game state account
 *
 * Data:
 * - winner: {@link PublicKey} The winner of the battle
 * - total_questions: {@link number} Total questions in the battle
 * - player_a_correct: {@link number} Correct answers by player A
 * - player_b_correct: {@link number} Correct answers by player B
 * - battle_duration: {@link number} Battle duration in seconds
 * - critical_hits: {@link number} Total critical hits
 */
export const settleBattleResultsSendAndConfirm = async (
  args: Omit<SettleBattleResultsArgs, "feePayer" | "settlementAuthority"> & {
    signers: {
      feePayer: web3.Signer,
      settlementAuthority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return settleBattleResultsBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      settlementAuthority: args.signers.settlementAuthority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.settlementAuthority])
    .rpc();
}

export type UpdateGameConfigArgs = {
  feePayer: web3.PublicKey;
  authority: web3.PublicKey;
  newWarriorCreationFee: bigint;
  newBattleEntryFee: bigint;
  newVrfOracle: web3.PublicKey;
};

/**
 * ### Returns a {@link MethodsBuilder}
 * Update game configuration (admin only)
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} The game authority
 * 2. `[writable]` game_state: {@link GameState} The global game state account
 *
 * Data:
 * - new_warrior_creation_fee: {@link BigInt} New warrior creation fee (optional)
 * - new_battle_entry_fee: {@link BigInt} New battle entry fee (optional)
 * - new_vrf_oracle: {@link PublicKey} New VRF oracle (optional)
 */
export const updateGameConfigBuilder = (
	args: UpdateGameConfigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): MethodsBuilder<RustUndead, never> => {
  const [gameStatePubkey] = pda.deriveGameStateSeedPDA(_program.programId);

  return _program
    .methods
    .updateGameConfig(
      new BN(args.newWarriorCreationFee.toString()),
      new BN(args.newBattleEntryFee.toString()),
      args.newVrfOracle,
    )
    .accountsStrict({
      feePayer: args.feePayer,
      authority: args.authority,
      gameState: gameStatePubkey,
    })
    .remainingAccounts(remainingAccounts);
};

/**
 * ### Returns a {@link web3.TransactionInstruction}
 * Update game configuration (admin only)
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} The game authority
 * 2. `[writable]` game_state: {@link GameState} The global game state account
 *
 * Data:
 * - new_warrior_creation_fee: {@link BigInt} New warrior creation fee (optional)
 * - new_battle_entry_fee: {@link BigInt} New battle entry fee (optional)
 * - new_vrf_oracle: {@link PublicKey} New VRF oracle (optional)
 */
export const updateGameConfig = (
	args: UpdateGameConfigArgs,
	remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionInstruction> =>
    updateGameConfigBuilder(args, remainingAccounts).instruction();

/**
 * ### Returns a {@link web3.TransactionSignature}
 * Update game configuration (admin only)
 *
 * Accounts:
 * 0. `[signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` authority: {@link PublicKey} The game authority
 * 2. `[writable]` game_state: {@link GameState} The global game state account
 *
 * Data:
 * - new_warrior_creation_fee: {@link BigInt} New warrior creation fee (optional)
 * - new_battle_entry_fee: {@link BigInt} New battle entry fee (optional)
 * - new_vrf_oracle: {@link PublicKey} New VRF oracle (optional)
 */
export const updateGameConfigSendAndConfirm = async (
  args: Omit<UpdateGameConfigArgs, "feePayer" | "authority"> & {
    signers: {
      feePayer: web3.Signer,
      authority: web3.Signer,
    },
  },
  remainingAccounts: Array<web3.AccountMeta> = [],
): Promise<web3.TransactionSignature> => {
  const preInstructions: Array<web3.TransactionInstruction> = [];


  return updateGameConfigBuilder({
      ...args,
      feePayer: args.signers.feePayer.publicKey,
      authority: args.signers.authority.publicKey,
    }, remainingAccounts)
    .preInstructions(preInstructions)
    .signers([args.signers.feePayer, args.signers.authority])
    .rpc();
}

// Getters

export const getGameState = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<RustUndead>["gameState"]> => _program.account.gameState.fetch(publicKey, commitment);

export const getWarrior = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<RustUndead>["warrior"]> => _program.account.warrior.fetch(publicKey, commitment);

export const getBattleRoom = (
    publicKey: web3.PublicKey,
    commitment?: web3.Commitment
): Promise<IdlAccounts<RustUndead>["battleRoom"]> => _program.account.battleRoom.fetch(publicKey, commitment);
