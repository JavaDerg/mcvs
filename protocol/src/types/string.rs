use crate::codec::{DecodeError, EncodeError, Transcodeable};
use crate::types::VarInt;
use bytes::{Buf, BufMut};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub const MAX_CHARS_STR: usize = 0x7FFF;

pub const MAX_CHARS_CHAT: usize = 0x40000;

pub const MAX_CHARS_IDENT: usize = 0x7FFF;

pub type MaxString = StringN<MAX_CHARS_STR>;
pub type Identifier = StringN<MAX_CHARS_IDENT>;

#[derive(Clone)]
pub struct StringN<const N: usize>(pub Cow<'static, str>);

impl<const N: usize> Transcodeable for StringN<N> {
    fn encode<B: BufMut>(&self, mut buf: B) -> Result<(), EncodeError> {
        let str = self.0.as_ref();
        let len = str.len();
        if len > N * 4 || (cfg!(debug_assertions) && str.chars().count() > N) {
            return Err(EncodeError::InputToLong);
        }

        VarInt(len as i32).encode(&mut buf)?;
        buf.put_slice(str.as_bytes());

        Ok(())
    }

    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
        let len = *(VarInt::decode(&mut buf)?) as usize;
        if len > N {
            return Err(DecodeError::OversizeString { max: N, recv: len });
        }
        if buf.remaining() < len {
            return Err(DecodeError::ToLittleData);
        }
        let data = (&buf.chunk()[..len]).to_vec();
        buf.advance(len);

        let string = String::from_utf8(data).map_err(|_| DecodeError::InvalidData)?;
        Ok(StringN(Cow::Owned(string)))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(
            VarInt(self.0.len() as i32)
                .size_hint()
                .expect("VarInt should always return Some on size_hint()")
                + self.0.len(),
        )
    }
}

impl<const N: usize> Deref for StringN<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<const N: usize> Debug for StringN<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
