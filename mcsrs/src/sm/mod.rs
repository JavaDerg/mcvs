use bytes::Bytes;
use protocol::codec::DecodeError;
use protocol::codec::Transcodeable;
use protocol::packets::{self, encode};
use protocol::types::{Chat, ChatObj};
use tracing::*;

pub mod encryption;

macro_rules! receive {
    ($val:expr => $type:ty) => {{
        debug!("receiving packet; kind={}", stringify!($type));
        match protocol::packets::decode::<$type, _>($val) {
            Ok(pkt) => pkt,
            Err(err) => {
                error!(err = %err);
                return Err(err);
            }
        }}
    }
}

macro_rules! respond {
    ($self:ident $(<< $val:expr)*) => {
        $($self
            .send_queue
            .send(protocol::packets::encode($val).expect("TODO: ADD GENERAL ERROR HANDLING!"))
            .expect("TODO: add this to error handling");)*
    };
}

pub struct StateMachine {
    encryption: Box<dyn encryption::Encryption + Send + Sync>,
    compressed: bool,
    send_queue: tokio::sync::mpsc::UnboundedSender<Bytes>,
    state: State,
}

#[derive(Debug)]
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
        packet = self.encryption.decrypt(packet);
        if self.compressed {
            packet = protocol::packets::decompess(packet)?
        }

        debug!(
            "selecting state; state={:?}; pkt={:?}",
            &self.state, &packet
        );
        match &self.state {
            State::Init => self.handshake(packet)?,
            State::Status(0) => self.status0(packet)?,
            State::Status(1) => self.status1(packet)?,
            State::Status(_) => unreachable!(),
            State::Login(_) => todo!(),
        }

        Ok(())
    }

    fn handshake(&mut self, packet: Bytes) -> Result<(), DecodeError> {
        let packets::Handshake {
            version,
            address,
            port,
            next,
        } = receive!(packet => packets::Handshake);
        debug!(
            "v={}, addr={}, port={}, next={}",
            *version, &*address, port, *next
        );
        match *next {
            1 => self.state = State::Status(0),
            2 => {
                self.state = State::Login(0);
            }
            _ => return Err(DecodeError::InvalidData),
        }
        Ok(())
    }

    fn status0(&mut self, packet: Bytes) -> Result<(), DecodeError> {
        use packets::status::{
            clientbound::{response::*, StatusResponse},
            serverbound::StatusRequest,
        };

        // StatusRequest is Zero Sized, this will only act as gate
        receive!(packet => StatusRequest);
        respond!(
            self << StatusResponse(Response {
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
            })
        );

        if let State::Status(n) = &mut self.state {
            *n = 1;
        }

        Ok(())
    }

    fn status1(&mut self, packet: Bytes) -> Result<(), DecodeError> {
        use packets::status::{clientbound::StatusPong, serverbound::StatusPing};

        let pkt = receive!(packet => StatusPing);
        respond!(self << StatusPong(pkt.0));

        Ok(())
    }
}
