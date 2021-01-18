use std::net::SocketAddr;

pub use super::{Packet, PacketDecode, PacketEncode};
pub use crate::{types::Magic, utils::BinaryStream, consts};
//TODO: Add prelude?

pub struct FirstOpenConnectionRequest {
    pub magic: Magic,
    pub protocol_version: u8,
    pub mtu_lenght: u16
}

impl PacketDecode for FirstOpenConnectionRequest {
    fn decode(packet: &mut Packet) -> Self {
        Self {
            magic: packet.read_magic(),
            protocol_version: packet.stream.read(),
            mtu_lenght: packet.stream.data.len() as u16
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
        packet.add_magic(consts::MAGIC);
        packet.stream.add(consts::SERVER_GUID);
        packet.stream.add(self.use_security);
        packet.stream.add(self.mtu_length);

        packet
    }
}


#[derive(Debug)]
pub struct SecondOpenConnectionRequest {
    pub magic: Magic,
    pub server_address: SocketAddr,
    pub mtu_length: u16,
    pub client_guid: u64
}

impl PacketDecode for SecondOpenConnectionRequest {
    fn decode(packet: &mut Packet) -> Self {
        Self {
            magic: packet.read_magic(),
            server_address: packet.read_address(),
            mtu_length: packet.stream.read(),
            client_guid: packet.stream.read()
        }
    }
}

pub struct SecondOpenConnectionReply {
    pub client_address: SocketAddr,
    pub mtu_length: u16,
    pub enctyption: bool
}

impl SecondOpenConnectionReply {
    pub fn new(client_address: SocketAddr, mtu_length: u16, enctyption: bool) -> Self {
        Self { client_address, mtu_length, enctyption }
    }
}

impl PacketEncode for SecondOpenConnectionReply {
    fn encode(&self) -> Packet {
        let mut packet = Packet::new(0x08, 16 + 8 + 7 + 2 + 1);

        packet.add_magic(consts::MAGIC);
        packet.stream.add(consts::SERVER_GUID);
        packet.add_address(self.client_address);
        packet.stream.add(self.mtu_length);
        packet.stream.add(self.enctyption);

        packet
    }
}