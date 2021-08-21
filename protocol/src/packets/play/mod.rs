mod dim;

use crate::types::{Array, Identifier, VarInt};
use protocol_derive::packet;

#[packet(id = 0x01)]
pub struct SpawnExperienceOrb {
    pub id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}

#[packet(id = 0x26)]
pub struct JoinGame {
    pub eid: VarInt,
    pub hardcore: bool,
    pub gamemode: u8,
    pub prev_gamemode: i8,
    pub worlds: Array<Identifier, VarInt>,
    pub dim_codec: nbt::Blob,
    pub dim: nbt::Blob,
    pub world_name: Identifier,
    pub hashed_seed: i64,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub reduce_debug: bool,
    pub respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
}
