pub use super::{Packet, PacketDecode, PacketEncode};
pub use crate::{types::Magic, utils::BinaryStream, consts};

pub struct OfflinePingPacket {
    pub time: u64,
    pub magic: Magic,
    pub client_guid: u64
}

//TODO: Rewrite it to proc-macro
impl PacketDecode for OfflinePingPacket {
    fn decode(bstream: &mut BinaryStream) -> Self {
        OfflinePingPacket {
            time: bstream.read(),
            magic: bstream.read_magic(),
            client_guid: bstream.read()
        }
    }
}

pub struct OfflinePongPacket {
    pub time: u64,
    pub server_id_string: String //TODO: Add own string structure
}

impl OfflinePongPacket {
    pub fn new(time: u64, server_id_string: String) -> Self {
        Self { time, server_id_string }
    }
}

impl PacketEncode for OfflinePongPacket {
    fn encode(&self) -> Packet {
        let mut packet = Packet::new(0x1c, 8 + 8 + 16 + (2 + self.server_id_string.len()));
        
        packet.stream.add(self.time);
        packet.stream.add(consts::SERVER_GUID);
        packet.stream.add_magic(consts::MAGIC);
        packet.stream.add(self.server_id_string.len() as u16);
        packet.stream.add_slice(&self.server_id_string.as_bytes());

        packet
    }
}