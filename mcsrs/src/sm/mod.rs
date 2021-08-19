use bytes::Bytes;
use protocol::codec::DecodeError;
use protocol::codec::Transcodeable;
use protocol::packets::{self, encode};
use protocol::types::{Chat, ChatObj};
use tracing::*;

pub mod encryption;

pub struct StateMachine {
    encryption: Box<dyn encryption::Encryption + Send + Sync>,
    compressed: bool,
    send_queue: tokio::sync::mpsc::UnboundedSender<Bytes>,
    state: State,
}

enum State {
    Init,
    Status(u8),
    Login(u8),
}

impl StateMachine {
    pub fn new(send_queue: tokio::sync::mpsc::UnboundedSender<Bytes>) -> Self {
        Self {
            encryption: Box::new(encryption::PassTrough),
            compressed: false,
            send_queue,
            state: State::Init,
        }
    }

    pub async fn submit(&mut self, mut packet: Bytes) -> Result<(), DecodeError> {
        debug!("decrypting packet; pkt={:?}", &packet);
        packet = self.encryption.decrypt(packet);
        debug!("decompressing packet; pkt={:?}", &packet);
        if self.compressed {
            packet = protocol::packets::decompess(packet)?
        }

        debug!("selecting state; pkt={:?}", &packet);
        match &self.state {
            State::Init => self.handshake(packet).await?,
            State::Status(_) => {}
            State::Login(_) => {}
        }

        Ok(())
    }

    async fn handshake(&mut self, packet: Bytes) -> Result<(), DecodeError> {
        debug!("reading handshake");
        let pkt = match protocol::packets::decode::<packets::Handshake, _>(packet) {
            Ok(pkt) => pkt,
            Err(err) => {
                error!(err = %err);
                return Err(err);
            }
        };
        let packets::Handshake {
            version,
            address,
            port,
            next,
        } = pkt;
        debug!(
            "v={}, addr={}, port={}, next={}",
            *version, &*address, port, *next
        );

        match *next {
            1 => {
                use packets::status::clientbound::{response::*, StatusResponse};
                self.state = State::Status(0);
                debug!("Sending status response");
                self.send_queue
                    .send(
                        encode(StatusResponse(Response {
                            version: Version {
                                name: protocol::MC_VERSION.to_string(),
                                protocol: protocol::VERSION,
                            },
                            players: Players {
                                max: 1337,
                                online: 0,
                                sample: None,
                            },
                            description: Chat::Obj(ChatObj {
                                text: Some("MCSRS!".to_string()),
                                translate: None,
                                score: None,
                                keybind: None,
                                selector: None,
                                bold: Some(true),
                                italic: None,
                                underlined: None,
                                strikethrough: None,
                                obfuscated: None,
                                color: Some("gold".to_string()),
                                insertion: None,
                                click_event: None,
                                hover_event: None,
                                extra: None,
                            }),
                            favicon: None,
                        }))
                        .expect("TODO: ADD GENERAL ERROR HANDLING!"),
                    )
                    .unwrap();
            }
            2 => {
                self.state = State::Login(0);
                todo!()
            }
            _ => return Err(DecodeError::InvalidData),
        }

        Ok(())
    }
}
