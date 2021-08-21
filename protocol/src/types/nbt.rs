use crate::codec::{DecodeError, EncodeError, Transcodeable};
use crate::iob::{BufMutWrapper, BufWrapper};
use bytes::{Buf, BufMut};

impl Transcodeable for nbt::Blob {
    fn encode<B: BufMut>(&self, buf: B) -> Result<(), EncodeError> {
        nbt::to_writer(&mut BufMutWrapper(buf), self, None).map_err(EncodeError::NbtError)
    }

    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError> {
        nbt::from_reader(&mut BufWrapper(buf)).map_err(|_| DecodeError::InvalidData)
    }
}
