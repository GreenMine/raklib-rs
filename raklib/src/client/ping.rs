use crate::client::{Client, Error};
use crate::protocol::packets::offline::{OfflinePingPacket, OfflinePongPacket};
use raklib_std::packet::Packet;
use raklib_std::protocol::types::MAGIC;
use raklib_std::stream::BinaryStream;
use tokio::net::ToSocketAddrs;

// FIXME: move just to client
pub async fn ping<A: ToSocketAddrs>(address: A) -> Result<OfflinePongPacket, Error> {
    let mut client = Client::bind(address).await?;

    client
        .send(&OfflinePingPacket {
            time: 0,
            magic: MAGIC,
            client_guid: 0,
        })
        .await
        .unwrap();

    let mut bstream = BinaryStream::with_len(2048);
    loop {
        if let Ok((read_bytes, _)) = client.socket.recv_from(bstream.get_raw_mut()).await {
            bstream.get_raw_mut().truncate(read_bytes); //FIXME: truncate free truncated elements memory block
            let packet_id = bstream.read::<u8>().unwrap();
            match packet_id {
                OfflinePongPacket::ID => {
                    // FIXME: forget about error
                    let pong = bstream.decode::<OfflinePongPacket>()?;

                    return Ok(pong);
                }
                _ => {
                    log::error!("Unexpected packet: 0x{:02X}", packet_id);
                }
            }
        }
    }
}
