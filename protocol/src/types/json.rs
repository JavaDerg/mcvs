use crate::codec::{DecodeError, EncodeError, Transcodeable};
use crate::types::StringN;
use bytes::{Buf, BufMut};
use std::borrow::{BorrowMut, Cow};
use std::fmt::{Debug, Formatter};

pub struct JsonN<T: Clone, const N: usize>(pub T);
pub struct JsonRef<'a, T: Clone, const N: usize>(pub Cow<'a, T>);

pub type Json<T> = JsonN<T, { crate::types::MAX_CHARS_STR }>;

impl<T, const N: usize> Transcodeable for JsonN<T, N>
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    fn encode<B: BufMut>(&self, buf: B) -> Result<(), EncodeError> {
        JsonRef::<T, N>(Cow::Borrowed(&self.0)).encode(buf)
    }

    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError> {
        if let Cow::Owned(obj) = JsonRef::<T, N>::decode(buf)?.0 {
            Ok(JsonN(obj))
        } else {
            unreachable!()
        }
    }
}

impl<'a, T, const N: usize> Transcodeable for JsonRef<'a, T, N>
where
    T: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    fn encode<B: BufMut>(&self, buf: B) -> Result<(), EncodeError> {
        let json_str = simd_json::to_string(&self.0).map_err(EncodeError::JsonError)?;
        StringN::<N>(Cow::Owned(json_str)).encode(buf)
    }

    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError> {
        let str = StringN::<N>::decode(buf)?;
        let mut str = str.0.into_owned();
        let obj: T = simd_json::from_str(str.borrow_mut()).map_err(|_| DecodeError::InvalidData)?;
        Ok(Self(Cow::Owned(obj)))
    }
}

impl<T: Debug + Clone, const N: usize> Debug for JsonN<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
