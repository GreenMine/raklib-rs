use std::net::SocketAddr;

use flate2::{Decompress, FlushDecompress};

use raklib::server::Server;
use raklib_std::{packet::Packet, stream::BinaryStream};
use tokio::sync::mpsc::Receiver;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let address = "0.0.0.0:19135".parse().unwrap();
    let mut server = Server::bind(address).await.unwrap();

    while let Some((socket, addr)) = server.accept().await {
        tokio::spawn(async move { handle_client(socket, addr).await });
    }
}

async fn handle_client(mut socket: Receiver<BinaryStream>, addr: SocketAddr) {
    while let Some(binary) = socket.recv().await {
        let data = binary.get_data();

        println!("new data(len: {})", data.len());
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
