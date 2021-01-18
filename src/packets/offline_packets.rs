pub use super::{Packet, PacketDecode, PacketEncode};
use crate::types::RakNetString;
pub use crate::{types::Magic, utils::BinaryStream, consts};

pub struct OfflinePingPacket {
    pub time: u64,
    pub magic: Magic,
    pub client_guid: u64
}

//TODO: Rewrite it to proc-macro
impl PacketDecode for OfflinePingPacket {
    fn decode(packet: &mut Packet) -> Self {
        OfflinePingPacket {
            time: packet.stream.read(),
            magic: packet.read_magic(),
            client_guid: packet.stream.read()
        }
    }
}

pub struct OfflinePongPacket<'a> {
    pub time: u64,
    pub server_id_string: RakNetString<'a>
}

impl<'a> OfflinePongPacket<'a> {
    pub fn new(time: u64, server_id_string: &'a String) -> Self {
        Self { time, server_id_string: RakNetString::from_string(server_id_string) }
    }
}

impl<'a> PacketEncode for OfflinePongPacket<'a> {
    fn encode(&self) -> Packet {
        let mut packet = Packet::new(0x1c, 8 + 8 + 16 + (2 + self.server_id_string.length as usize));
        
        packet.stream.add(self.time);
        packet.stream.add(consts::SERVER_GUID);
        packet.add_magic(consts::MAGIC);
        packet.add_string(&self.server_id_string);

        packet
    }
}