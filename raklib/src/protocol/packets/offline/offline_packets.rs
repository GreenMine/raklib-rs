use raklib_std::packet::{Packet, PacketDecode};
use raklib_std::protocol::types::{self, Magic, RakNetString};
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

#[derive(raklib_std::derive::PacketEncode)]
pub struct OfflinePongPacket<'a> {
    pub time: u64,
    #[const_fields(consts::SERVER_GUID, types::MAGIC)]
    pub server_id_string: RakNetString<'a>,
}

impl<'a> OfflinePongPacket<'a> {
    pub fn new(time: u64, server_id_string: &'a str) -> Self {
        Self {
            time,
            server_id_string: RakNetString::from_string(server_id_string),
        }
    }
}

impl<'a> PacketDecode for OfflinePongPacket<'a> {
    fn decode(bstream: &mut BinaryStream) -> raklib_std::stream::Result<Self>
    where
        Self: Sized,
    {
        let time: u64 = bstream.read()?;
        let _: u64 = bstream.read()?;
        let _: Magic = bstream.read()?;
        let server_id_string: RakNetString = bstream.read()?;

        Ok(Self {
            time,
            server_id_string,
        })
    }
}

impl<'a> Packet for OfflinePongPacket<'a> {
    const ID: u8 = 0x1c;
    fn packet_size(&self) -> usize {
        1 + 8 + 8 + 16 + (2 + self.server_id_string.length as usize)
    }
}
