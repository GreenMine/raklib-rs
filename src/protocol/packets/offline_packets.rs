use super::{Packet, PacketEncode, PacketDecode};
use crate::{protocol::{types::{Magic, RakNetString}, consts}, utils::BinaryStream};

pub struct OfflinePingPacket {
    pub time: u64,
    pub magic: Magic,
    pub client_guid: u64
}

impl Packet for OfflinePingPacket {
    const ID: u8 = 0x01;
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

pub struct OfflinePongPacket<'a> {
    pub time: u64,
    pub server_id_string: RakNetString<'a>
}

impl<'a> OfflinePongPacket<'a> {
    pub fn new(time: u64, server_id_string: &'a String) -> Self {
        Self { time, server_id_string: RakNetString::from_string(server_id_string) }
    }
}

impl<'a> Packet for OfflinePongPacket<'a> {
    const ID: u8 = 0x1c;
    fn packet_size(&self) -> usize { 8 + 8 + 16 + (2 + self.server_id_string.length as usize) }
}

impl<'a> PacketEncode for OfflinePongPacket<'a> {
    fn encode_payload(&self, bstream: &mut BinaryStream) {
        bstream.add(self.time);
        bstream.add(consts::SERVER_GUID);
        bstream.add_magic(consts::MAGIC);
        bstream.add_string(&self.server_id_string);
    }
}