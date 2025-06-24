import {PublicKey} from "@solana/web3.js";
import {BN} from "@coral-xyz/anchor";

export const deriveGameStateSeedPDA = (programId: PublicKey): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("game_state"),
        ],
        programId,
    )
};

export type WarriorSeedSeeds = {
    owner: PublicKey, 
    name: string, 
};

export const deriveWarriorSeedPDA = (
    seeds: WarriorSeedSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("warrior"),
            seeds.owner.toBuffer(),
            Buffer.from(seeds.name, "utf8"),
        ],
        programId,
    )
};

export type BattleRoomSeedSeeds = {
    roomId: string, 
};

export const deriveBattleRoomSeedPDA = (
    seeds: BattleRoomSeedSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("battle_room"),
            Buffer.from(seeds.roomId, "utf8"),
        ],
        programId,
    )
};

