pub mod clientbound {
    use protocol_derive::packet;

    #[packet(id = 0x00)]
    #[derive(Debug)]
    pub struct StatusResponse(pub response::Response);

    #[packet(id = 0x01)]
    #[derive(Debug)]
    pub struct StatusPong {
        payload: i64,
    }

    pub mod response {
        use crate::types::Chat;
        use protocol_derive::Json;
        use uuid::Uuid;

        #[derive(Json, Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct Response {
            pub version: Version,
            pub players: Players,
            pub description: Chat,
            pub favicon: Option<String>,
        }

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct Version {
            pub name: String,
            pub protocol: i32,
        }

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct Players {
            pub max: i32,
            pub online: i32,
            pub sample: Option<Vec<Player>>,
        }

        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
        pub struct Player {
            pub name: String,
            pub id: Uuid,
        }
    }
}

pub mod serverbound {
    use protocol_derive::packet;

    #[packet(id = 0x00)]
    #[derive(Debug)]
    pub struct StatusRequest;

    #[packet(id = 0x01)]
    #[derive(Debug)]
    pub struct StatusPing {
        payload: i64,
    }
}
