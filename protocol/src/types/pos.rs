use crate::codec::{DecodeError, EncodeError, Transcodeable};
use bytes::{Buf, BufMut};

pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Transcodeable for Position {
    fn encode<B: BufMut>(&self, buf: B) -> Result<(), EncodeError> {
        let x = self.x as u64;
        let y = self.y as u64;
        let z = self.z as u64;
        let combo = ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF);
        (combo as i64).encode(buf)
    }

    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError> {
        let combo = i64::decode(buf)? as u64;
        let mut x = (combo >> 38) as i32;
        let mut y = (combo & 0xFFF) as i32;
        let mut z = (combo << 26 >> 38) as i32;
        if x >= 2i32.pow(25) {
            x -= 2i32.pow(26)
        }
        if y >= 2i32.pow(11) {
            y -= 2i32.pow(12)
        }
        if z >= 2i32.pow(25) {
            z -= 2i32.pow(26)
        }
        Ok(Self { x, y, z })
    }

    fn size_hint(&self) -> Option<usize> {
        Some(8)
    }
}
