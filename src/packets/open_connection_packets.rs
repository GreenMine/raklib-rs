pub use super::{Packet, PacketDecode, PacketEncode};
pub use crate::{types::Magic, utils::BinaryStream, consts};
//TODO: Add prelude?

pub struct FirstOpenConnectionRequest {
    pub magic: Magic,
    pub protocol_version: u8,
    pub mtu_lenght: u16
}

impl PacketDecode for FirstOpenConnectionRequest {
    fn decode(bstream: &mut BinaryStream) -> Self {
        Self {
            magic: bstream.read_magic(),
            protocol_version: bstream.read(),
            mtu_lenght: bstream.data.len() as u16
        }
    }
}

pub struct FirstOpenConnectionReply {
    pub use_security: bool,
    pub mtu_length: u16
}

impl FirstOpenConnectionReply {
    pub fn new(use_security: bool, mtu_length: u16) -> Self {
        Self { use_security, mtu_length }
    }
}

impl PacketEncode for FirstOpenConnectionReply {
    fn encode(&self) -> Packet {
        let mut packet = Packet::new(0x06, 16 + 8 + 1 + 2);
        packet.stream.add_magic(consts::MAGIC);
        packet.stream.add(consts::SERVER_GUID);
        packet.stream.add(self.use_security);
        packet.stream.add(self.mtu_length);

        packet
    }
}


pub struct SecondOpenConnectionRequest {
}

impl PacketDecode for SecondOpenConnectionRequest {
    fn decode(bstream: &mut BinaryStream) -> Self {
        Self {
        }
    }
}

pub struct SecondOpenConnectionReply {
    pub use_security: bool,
    pub mtu_length: u16
}

impl SecondOpenConnectionReply {
    pub fn new(use_security: bool, mtu_length: u16) -> Self {
        Self { use_security, mtu_length }
    }
}

impl PacketEncode for SecondOpenConnectionReply {
    fn encode(&self) -> Packet {
        let mut packet = Packet::new(0x06, 16 + 8 + 1 + 2);
        packet.stream.add_magic(consts::MAGIC);
        packet.stream.add(consts::SERVER_GUID);
        packet.stream.add(self.use_security);
        packet.stream.add(self.mtu_length);

        packet
    }
}