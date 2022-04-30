use flate2::{Decompress, FlushDecompress};
use raklib::Server;
use raklib_std::packet::Packet;

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:19135".parse().unwrap();
    let mut server = Server::bind(address).await.unwrap();
    server.run().await.unwrap();

    loop {
        if let Some((user, mut listener)) = server.recv().await {
            tokio::spawn(async move {
                loop {
                    if let Some(stream) = listener.recv().await {
                        println!("new data");

                        let data = stream.get_data();
                        let mut decompress = Decompress::new(false);
                        let mut decompressed = Vec::with_capacity(65536);
                        decompress
                            .decompress_vec(&data, &mut decompressed, FlushDecompress::Finish)
                            .unwrap();

                        std::fs::write(
                            "log/decompressed.bin",
                            &decompressed[..decompress.total_out() as usize],
                        )
                        .unwrap();
                    }
                }
            });
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
