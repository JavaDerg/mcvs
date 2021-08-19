use std::error::Error;
use std::fmt::{Display, Formatter};

use bytes::{Buf, BufMut};

#[derive(Debug)]
pub enum DecodeError {
    InvalidData,
    ToLittleData,
    OversizeString { max: usize, recv: usize },
}

#[derive(Debug)]
pub enum EncodeError {
    InputToLong,
    JsonError(simd_json::Error),
}

pub trait Packet: Transcodeable {
    fn id() -> i32;
    fn id_self(&self) -> i32 {
        Self::id()
    }
}

pub trait Transcodeable: Sized {
    fn encode<B: BufMut>(&self, buf: B) -> Result<(), EncodeError>;
    fn decode<B: Buf>(buf: B) -> Result<Self, DecodeError>;
    fn size_hint(&self) -> Option<usize> {
        None
    }
}

impl Transcodeable for bool {
    fn encode<B: BufMut>(&self, mut buf: B) -> Result<(), EncodeError> {
        buf.put_u8(if *self { 0x01 } else { 0x00 });
        Ok(())
    }

    fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
        match buf.remaining() {
            0 => Err(DecodeError::ToLittleData),
            _ => Ok(match buf.get_u8() {
                0 => false,
                1 => true,
                _ => return Err(DecodeError::InvalidData),
            }),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(1)
    }
}

macro_rules! impl_int {
    ($([$type:ident, $encode:ident, $decode:ident];)*) => {
        $(
            impl Transcodeable for $type {
                fn encode<B: BufMut>(&self, mut buf: B) -> Result<(), EncodeError> {
                    buf.$encode(*self);
                    Ok(())
                }

                fn decode<B: Buf>(mut buf: B) -> Result<Self, DecodeError> {
                    const MAX: usize = std::mem::size_of::<$type>() - 1;
                    match buf.remaining() {
                        0..=MAX => Err(DecodeError::ToLittleData),
                        _ => Ok(buf.$decode()),
                    }
                }

                fn size_hint(&self) -> Option<usize> {
                    Some(std::mem::size_of::<$type>())
                }
            }
        )*
    };
}

impl_int! {
    [u8, put_u8, get_u8];
    [i8, put_i8, get_i8];
    [u16, put_u16, get_u16];
    [i16, put_i16, get_i16];
    [i32, put_i32, get_i32];
    [i64, put_i64, get_i64];

    [f32, put_f32, get_f32];
    [f64, put_f64, get_f64];
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for EncodeError {}

impl Error for DecodeError {}
