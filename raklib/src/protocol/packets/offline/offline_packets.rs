use raklib_std::packet::{Packet, PacketDecode};
use raklib_std::protocol::types::{self, Magic};
use raklib_std::stream::BinaryStream;

use crate::protocol::consts;

#[derive(raklib_std::derive::PacketDecode, raklib_std::derive::PacketEncode)]
pub struct OfflinePingPacket {
    pub time: u64,
    pub magic: Magic,
    pub client_guid: u64,
}

impl Packet for OfflinePingPacket {
    const ID: u8 = 0x01;
}

pub struct OfflinePongPacket {
    pub time: u64,
    pub server_id_string: String,
}

impl OfflinePongPacket {
    pub fn new(time: u64, server_id_string: &str) -> Self {
        Self {
            time,
            server_id_string: server_id_string.to_string(),
        }
    }
}

impl raklib_std::packet::PacketEncode for OfflinePongPacket {
    fn encode_payload(&self, bstream: &mut raklib_std::stream::BinaryStream) {
        bstream.add(self.time);
        bstream.add(consts::SERVER_GUID);
        bstream.add(types::MAGIC);
        bstream.add(&self.server_id_string);
    }
}

impl PacketDecode for OfflinePongPacket {
    fn decode(bstream: &mut BinaryStream) -> raklib_std::stream::Result<Self>
    where
        Self: Sized,
    {
        let time: u64 = bstream.read()?;
        let _: u64 = bstream.read()?;
        let magic: Magic = bstream.read()?;

        log::info!("Magic validation: {}", magic.is_valid());

        let server_id_string: String = bstream.read()?;

        Ok(Self {
            time,
            server_id_string,
        })
    }
}

impl Packet for OfflinePongPacket {
    const ID: u8 = 0x1c;
    fn packet_size(&self) -> usize {
        1 + 8 + 8 + 16 + (2 + self.server_id_string.len())
    }
}
