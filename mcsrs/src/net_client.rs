use crate::sm::StateMachine;
use bytes::{Buf, Bytes, BytesMut};
use protocol::codec::{DecodeError, Transcodeable};
use protocol::types::VarInt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::*;
use tracing_futures::Instrument;

pub async fn handle_client(stream: TcpStream) {
    debug!("handeling new client");

    let (mut read, mut write) = stream.into_split();

    let (stx, mut srx) = tokio::sync::mpsc::unbounded_channel::<Bytes>();

    let mut write_handle = tokio::spawn(
        async move {
            debug!("spawned new write task");
            while let Some(mut packet) = srx.recv().await {
                debug!("received packet; pkt={:?}", &packet);
                loop {
                    match write.write_buf(&mut packet).await {
                        Ok(0) => return Ok(()),
                        Ok(_) if packet.has_remaining() => continue,
                        Ok(_) => break,
                        Err(err) => return Err(err),
                    }
                }
            }
            Ok(())
        }
        .instrument(debug_span!("write")),
    );

    debug!("creating state machine");
    let mut stmch = StateMachine::new(stx);

    let mut buf = BytesMut::with_capacity(1024);
    let mut clen = None;
    'read_loop: loop {
        debug!("waiting for read or join");
        let read = tokio::select! {
            read = read.read_buf(&mut buf) => match read {
                // Client closed connection
                Ok(0) => break,
                Ok(read) => read,
                Err(_err) => {
                    todo!("Handle error")
                }
            },
            _join_res = &mut write_handle => todo!(),
        };
        debug!("read data; len={}", read);

        loop {
            let len = match clen {
                Some(len) => len,
                None => {
                    let len_bytes = &(buf.chunk())[..5.min(buf.len())];
                    let len = match VarInt::decode(len_bytes) {
                        Ok(len) => *len as usize + len.size_hint().unwrap(),
                        Err(DecodeError::ToLittleData) => break,
                        Err(_) => unreachable!(),
                    };
                    clen = Some(len);
                    len
                }
            };

            debug!("decoded packet length; len={}", len);

            if buf.len() >= len {
                debug!("packet complete");
                let packet = buf.copy_to_bytes(len);
                match stmch.submit(packet).await {
                    Ok(_) => (),
                    Err(_err) => todo!("handle error"),
                }
                clen = None;
            } else {
                continue 'read_loop; // we break out of the processing loop as we have no complete packets left
            }
        }
    }
}
