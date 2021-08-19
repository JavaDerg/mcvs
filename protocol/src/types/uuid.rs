use crate::codec::{DecodeError, EncodeError, Transcodeable};
use bytes::{Buf, BufMut};
use uuid::Uuid;

impl Transcodeable for Uuid {
    fn encode<B: BufMut>(&self, mut buf: B) -> Result<(), EncodeError> {
        buf.put_u128(self.as_u128());
        Ok(())
    }

    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
        if buf.remaining() < 16 {
            return Err(DecodeError::ToLittleData);
        } else {
            Ok(Uuid::from_u128(buf.get_u128()))
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(16)
    }
}
