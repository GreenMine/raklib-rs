use flate2::{Decompress, FlushDecompress};

use raklib::Server;
use raklib_std::packet::Packet;

#[tokio::main]
async fn main() {
    env_logger::init();

    let address = "0.0.0.0:19135".parse().unwrap();
    let mut server = Server::bind(address).await.unwrap();
    server.run().await.unwrap();

    'main: loop {
        if let Some((user, mut listener)) = server.recv().await {
            loop {
                if let Some(stream) = listener.recv().await {
                    println!("new data");
                }
            }
        }
    }
}

#[derive(Debug, raklib_std::derive::PacketDecode, raklib_std::derive::PacketEncode)]
pub struct Login {
    foo: i32,
    bar: i32,
}

impl Packet for Login {
    const ID: u8 = 0x01;
}
