use raklib::client::Client;
use raklib::protocol::packets::offline::{OfflinePingPacket, OfflinePongPacket};
use raklib_std::protocol::types::MAGIC;
use raklib_std::stream::BinaryStream;
use std::time::Duration;

const SERVER_ADDR: &'static str = "0.0.0.0:19135";

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = SERVER_ADDR.parse().unwrap();
    let mut client = Client::connect(addr).await;

    log::info!("Successfully connected to {}!", SERVER_ADDR);

    client
        .send(&OfflinePingPacket {
            time: 0,
            magic: MAGIC,
            client_guid: 0,
        })
        .await
        .unwrap();

    let mut bstream = BinaryStream::with_len(2048);
    if let Ok((read_bytes, addr)) = client.socket.recv_from(bstream.get_raw_mut()).await {
        bstream.get_raw_mut().truncate(read_bytes); //FIXME: truncate free truncated elements memory block
        let pong = bstream.decode::<OfflinePongPacket>().unwrap();

        let server_id: &str = pong.server_id_string.try_into().unwrap();
        log::info!("Server id: {}", server_id);
    }
}
