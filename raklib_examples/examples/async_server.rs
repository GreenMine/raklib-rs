use flate2::{Decompress, FlushDecompress};

use raklib::server::Server;
use raklib_std::packet::Packet;

#[tokio::main]
async fn main() {
    env_logger::init();

    let address = "0.0.0.0:19135".parse().unwrap();
    let server = Server::bind(address).await.unwrap();
    let mut listener = server.run().await.unwrap();

    while let Some((user, mut session)) = listener.incoming().await {
        println!("New user: {}", user);
        loop {
            if let Some(stream) = session.recv().await {
                let data = stream.get_data();

                println!("new data(len: {})", data.len());
                println!("Packet id: 0x{:02X}", data[0]);
            }
        }
    }

    // 'main: loop {
    //     if let Some((user, mut listener)) = server.recv().await {
    //         loop {
    //             if let Some(stream) = listener.recv().await {
    //                 let data = stream.get_data();
    //
    //                 println!("new data(len: {})", data.len());
    //             }
    //         }
    //     }
    // }

    loop {}
}

#[derive(Debug, raklib_std::derive::PacketDecode, raklib_std::derive::PacketEncode)]
pub struct Login {
    foo: i32,
    bar: i32,
}

impl Packet for Login {
    const ID: u8 = 0x01;
}
