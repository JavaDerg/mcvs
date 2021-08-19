pub mod clientbound {
    use crate::types::{Array, Chat, StringN, VarInt};
    use protocol_derive::packet;
    use uuid::Uuid;

    #[packet(id = 0x00)]
    pub struct Disconnect(pub Chat);

    #[packet(id = 0x01)]
    pub struct EncryptionRequest {
        pub server_id: StringN<20>,
        pub public_key: Array<u8, VarInt>,
        pub verify_token: Array<u8, VarInt>,
    }

    #[packet(id = 0x02)]
    pub struct LoginSuccess {
        pub uuid: Uuid,
        pub username: StringN<16>,
    }

    #[packet(id = 0x03)]
    pub struct SetCompression(VarInt);
}

pub mod serverbound {
    use crate::types::{Array, StringN, VarInt};
    use protocol_derive::packet;

    #[packet(id = 0x00)]
    pub struct LoginStart(pub StringN<16>);

    #[packet(id = 0x01)]
    pub struct EncryptionResponse {
        pub shared_secret: Array<u8, VarInt>,
        pub verify_token: Array<u8, VarInt>,
    }
}
