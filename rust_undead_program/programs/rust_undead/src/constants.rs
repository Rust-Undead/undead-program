use anchor_lang::prelude::*;

pub const ANCHOR_DISCRIMINATOR : usize = 8;

#[constant]
pub const SEED: &str = "anchor";

pub const BATTLE_ROOM_SEED: &[u8] = b"battle_room";