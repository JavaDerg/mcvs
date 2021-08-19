use crate::codec::{DecodeError, EncodeError, SizeTranscodable, Transcodeable};
use bytes::{Buf, BufMut};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct Array<T: Transcodeable, I: SizeTranscodable>(pub Vec<T>, PhantomData<I>);

impl<T: Transcodeable, I: SizeTranscodable> Array<T, I> {
    pub fn new(t: Vec<T>) -> Self {
        Self(t, PhantomData)
    }
}

impl<T: Transcodeable, I: SizeTranscodable> Transcodeable for Array<T, I> {
    fn encode<B: BufMut>(&self, mut buf: B) -> Result<(), EncodeError> {
        I::encode_usize(self.0.len(), &mut buf)?;
        for item in &self.0 {
            item.encode(&mut buf);
        }
        Ok(())
    }

    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
        let len = I::decode_usize(&mut buf)?;
        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(T::decode(&mut buf)?);
        }

        Ok(Array(vec, PhantomData))
    }
}

impl<T: Transcodeable, I: SizeTranscodable> Deref for Array<T, I> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Transcodeable, I: SizeTranscodable> DerefMut for Array<T, I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
