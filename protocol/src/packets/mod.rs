pub mod login;
pub mod play;
pub mod status;

use crate::codec::{DecodeError, EncodeError, Packet, Transcodeable};
use crate::types::{StringN, VarInt};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use flate2::Compression;
use protocol_derive::packet;
use std::io::Read;
use tracing::*;

#[packet(id = 0x00)]
#[derive(Debug)]
pub struct Handshake {
    pub version: VarInt,
    pub address: StringN<255>,
    pub port: u16,
    pub next: VarInt,
}

pub fn encode(packet: impl Packet) -> Result<Bytes, EncodeError> {
    // if we know the size of the packet, we can allocate the right amount of memory beforehand
    Ok(match packet.size_hint() {
        Some(size_hint) => {
            let id = VarInt(packet.id_self());
            let size = VarInt(size_hint as i32 + id.size_hint().unwrap() as i32);
            let header_hint = size.size_hint().unwrap();

            let mut buf = BytesMut::with_capacity(header_hint + size_hint);
            size.encode(&mut buf)?;
            id.encode(&mut buf)?;
            packet.encode(&mut buf)?;

            buf
        }
        None => {
            let mut buf = BytesMut::with_capacity(64);
            packet.encode(&mut buf)?;
            let data = buf.freeze();

            let id = VarInt(packet.id_self());
            let size = VarInt(data.len() as i32 + id.size_hint().unwrap() as i32);

            let mut buf = BytesMut::with_capacity(data.len() + 8);
            size.encode(&mut buf)?;
            id.encode(&mut buf)?;
            buf.put(data);

            buf
        }
    }
    .freeze())
}

pub fn decode<P: Packet, B: Buf>(mut buf: B) -> Result<P, DecodeError> {
    let (mut len, id) = read_header(&mut buf)?;
    len -= VarInt(len as i32).size_hint().unwrap();
    if id != P::id() {
        return Err(DecodeError::InvalidData);
    }
    if buf.remaining() < len {
        return Err(DecodeError::ToLittleData);
    }
    P::decode((&mut buf).take(len))
}

pub fn read_header<B: Buf>(mut buf: B) -> Result<(usize, i32), DecodeError> {
    let len = VarInt::decode(&mut buf)?;
    let id = VarInt::decode(&mut buf)?;
    Ok((*len as usize, *id))
}

lazy_static::lazy_static! {
    pub static ref COMPRESSION_LEVEL: u32 = {
        std::env::var("MC_GZIP_LEVEL").map_or(4, |var| var.parse::<u32>().map(|v| match v {
            0..=9 => v,
            x => {
                eprintln!("MC_GZIP_LEVEL is set INVALIDLY, must be a number between 0-9; defaulting to 4");
                4
            }
        }).unwrap_or_else(|_| {
            eprintln!("MC_GZIP_LEVEL is set INVALIDLY, must be a number between 0-9; defaulting to 4");
            4
        }))
    };
}

pub fn compress(data: Bytes) -> Bytes {
    let mut cbuf = Vec::with_capacity(data.len());
    let mut zlib =
        flate2::bufread::ZlibEncoder::new(data.chunk(), Compression::new(*COMPRESSION_LEVEL));
    let _ = zlib
        .read_to_end(&mut cbuf)
        .expect("unable to compress data");

    let dlen = VarInt(data.len() as i32);
    let len = VarInt((cbuf.len() + dlen.size_hint().unwrap()) as i32);

    let mut buf = BytesMut::with_capacity(*len as usize + len.size_hint().unwrap());
    len.encode(&mut buf).unwrap();
    dlen.encode(&mut buf).unwrap();
    buf.put_slice(cbuf.as_slice());
    buf.freeze()
}

pub fn decompess(mut data: Bytes) -> Result<Bytes, DecodeError> {
    let (rl, dl) = read_compression_header(&mut data)?;
    if data.remaining() < (rl - VarInt(rl as i32).size_hint().unwrap()) {
        return Err(DecodeError::ToLittleData);
    }

    let mut buf = Vec::with_capacity(dl);
    let mut zlib = flate2::bufread::ZlibDecoder::new(data.chunk());
    let read = zlib
        .read_to_end(&mut buf)
        .map_err(|_| DecodeError::InvalidData)?;
    if read != dl {
        return Err(DecodeError::InvalidData);
    }

    Ok(Bytes::from(buf))
}

pub fn read_compression_header<B: Buf>(mut buf: B) -> Result<(usize, usize), DecodeError> {
    let len = VarInt::decode(&mut buf)?;
    let dlen = VarInt::decode(&mut buf)?;
    Ok((*len as usize, *dlen as usize))
}
