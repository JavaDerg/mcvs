#![deny(unsafe_code)]

mod net_client;
mod sm;

use tokio::net::TcpListener;
use tracing::*;
use tracing_futures::Instrument;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("0.0.0.0:25565").await.unwrap();
    loop {
        let client = listener.accept().await.unwrap();
        tokio::spawn(
            net_client::handle_client(client.0)
                .instrument(debug_span!("client", addr = client.1.to_string().as_str())),
        );
    }
}

/*
fn main() {
use bytes::{Bytes, BytesMut};
use protocol::codec::Transcodeable;
use protocol::types::{Chat, VarInt};
use std::borrow::BorrowMut;

    let buf = Bytes::from_static(b"\xff\x45\x05hello\x13\x37\x01");
                                // \x10\0\xf4\x05\tlocalhostc\xdd\x02
    let size = buf.len();
    println!("{:?}", &buf);
    let handshake = protocol::packets::Handshake::decode(buf).unwrap();
    println!("{:?}", &handshake);
    println!(
        "hint={:?}; real={}",
        Transcodeable::size_hint(&handshake),
        size
    );
    let mut buf = BytesMut::new();
    Transcodeable::encode(&handshake, &mut buf).unwrap();
    println!("{:?}", buf.freeze());

    println!("\n");

    println!(
        "{}",
        simd_json::to_string(&protocol::types::Chat::Primitive(
            protocol::types::Prim::Float(13.37)
        ))
        .unwrap()
    );

    println!("\n");

    let mut str = String::from(
        r#"["",{"text":"Welcome","italic":true},{"text":" to","italic":true},{"text":" "},{"text":"Minecraft ","clickEvent":{"action":"open_url","value":"https://minecraft.tools/en/tellraw.php"}},{"text":"Tools","color":"gold","clickEvent":{"action":"open_url","value":"https://minecraft.tools/en/tellraw.php"}},{"text":" "},{"keybind":"key.jump"},{"text":" "},{"score":{"name":"@a","objective":"deaths"}}]"#,
    );
    let chat: Chat = simd_json::from_str(str.borrow_mut()).unwrap();
    println!("{:?}", chat);
    println!("{}\n{}", str, simd_json::to_string(&chat).unwrap());

    println!("\n");

}
 */
