use raklib::client;
use raklib::protocol::packets::offline::{OfflinePingPacket, OfflinePongPacket};
use raklib_std::packet::Packet;
use raklib_std::protocol::types::MAGIC;
use raklib_std::stream::BinaryStream;

const SERVER_ADDR: &'static str = "51.222.46.166:19132";

#[tokio::main]
async fn main() {
    env_logger::init();

    let addr = SERVER_ADDR.parse().unwrap();
    let pong = client::ping(addr).await.unwrap();

    log::info!("Pong info:");
    log::info!("Time: {}", pong.time);
    log::info!("Server id: {}", pong.server_id_string);
}
