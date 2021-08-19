use crate::codec::{DecodeError, EncodeError, Transcodeable};
use bytes::{Buf, BufMut};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut, Div};
use tracing::field::debug;

#[derive(Copy, Clone)]
pub struct VarInt(pub i32);
#[derive(Copy, Clone)]
pub struct VarLong(pub i64);

/// https://wiki.vg/Protocol#VarInt_and_VarLong
/// https://en.wikipedia.org/wiki/LEB128
fn encode_leb128<B: BufMut>(mut num: u64, mut buf: B) -> Result<(), EncodeError> {
    loop {
        let mut cur = (num & 0x7F) as u8;
        num >>= 7;
        if num != 0 {
            cur |= 0x80;
        }
        buf.put_u8(cur);
        if cur >> 7 == 0 {
            break;
        }
    }
    Ok(())
}

fn decode_leb128<B: Buf>(mut buf: B, max_len: usize) -> Result<u64, DecodeError> {
    let mut shift = 0u64;
    let mut val = 0u64;

    let mut flag = false;

    while shift < 7 * max_len as u64 {
        if !buf.has_remaining() {
            return Err(DecodeError::ToLittleData);
        }
        let byte = buf.get_u8();

        val |= (byte as u64 & 0x7F) << shift;
        flag = byte >> 7 == 1;

        if !flag {
            break;
        }
        shift += 7;
    }

    if !flag {
        Ok(val)
    } else {
        Err(DecodeError::InvalidData)
    }
}

impl Transcodeable for VarInt {
    fn encode<B: BufMut>(&self, buf: B) -> Result<(), EncodeError> {
        // the as u32 as u64 is important to avoid a implicit conversation to i64 first
        encode_leb128(self.0 as u32 as u64, buf)
    }

    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError> {
        decode_leb128(buf, 5).map(|x| Self(x as i32))
    }

    fn size_hint(&self) -> Option<usize> {
        if self.0 == 0 {
            return Some(1);
        }

        let used = 32 - self.0.leading_zeros() as usize;
        let bytes = used / 7;
        Some(match used % 7 {
            0 => bytes,
            _ => bytes + 1,
        })
    }
}

impl Transcodeable for VarLong {
    fn encode<B: BufMut>(&self, buf: B) -> Result<(), EncodeError> {
        encode_leb128(self.0 as u64, buf)
    }

    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError> {
        decode_leb128(buf, 10).map(|x| Self(x as i64))
    }

    fn size_hint(&self) -> Option<usize> {
        let used = 64 - self.0.leading_zeros() as usize;
        let bytes = used / 7;
        Some(match used % 7 {
            0 => bytes,
            _ => bytes + 1,
        })
    }
}

impl Deref for VarInt {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VarInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for VarInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for VarLong {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VarLong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for VarLong {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
