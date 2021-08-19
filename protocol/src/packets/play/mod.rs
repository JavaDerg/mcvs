use crate::types::VarInt;
use protocol_derive::packet;

#[packet(id = 0x01)]
pub struct SpawnExperienceOrb {
    pub id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub count: i16,
}
