use std::net::SocketAddr;

use raklib_std::packet::{Packet, PacketDecode};
use raklib_std::protocol::types::{self, Magic};
use raklib_std::stream::{BinaryStream, Result};

use crate::protocol::consts;

//TODO: Add prelude?

pub struct FirstOpenConnectionRequest {
    pub magic: Magic,
    pub protocol_version: u8,
    pub mtu_length: u16,
}

impl Packet for FirstOpenConnectionRequest {
    const ID: u8 = 0x05;
}
impl PacketDecode for FirstOpenConnectionRequest {
    fn decode(bstream: &mut BinaryStream) -> Result<Self> {
        Ok(Self {
            magic: bstream.read()?,
            protocol_version: bstream.read()?,
            mtu_length: bstream.get_raw().len() as u16, //TODO: get stream length
        })
    }
}

#[derive(raklib_std::derive::PacketEncode)]
pub struct FirstOpenConnectionReply {
    #[const_fields(types::MAGIC, consts::SERVER_GUID)]
    pub use_security: bool,
    pub mtu_length: u16,
}

impl FirstOpenConnectionReply {
    pub fn new(use_security: bool, mtu_length: u16) -> Self {
        Self {
            use_security,
            mtu_length,
        }
    }
}

impl Packet for FirstOpenConnectionReply {
    const ID: u8 = 0x06;

    fn packet_size(&self) -> usize {
        1 + 16 + 8 + 1 + 2 // packet id + magic + server_guid + use security + mtu length
    }
}

#[derive(Debug, raklib_std::derive::PacketDecode)]
pub struct SecondOpenConnectionRequest {
    pub magic: Magic,
    pub server_address: SocketAddr,
    pub mtu_length: u16,
    pub client_guid: u64,
}

impl Packet for SecondOpenConnectionRequest {
    const ID: u8 = 0x07;
}

#[derive(raklib_std::derive::PacketEncode)]
pub struct SecondOpenConnectionReply {
    #[const_fields(types::MAGIC, consts::SERVER_GUID)]
    pub client_address: SocketAddr,
    pub mtu_length: u16,
    pub encryption: bool,
}

impl SecondOpenConnectionReply {
    pub fn new(client_address: SocketAddr, mtu_length: u16, encryption: bool) -> Self {
        Self {
            client_address,
            mtu_length,
            encryption,
        }
    }
}

impl Packet for SecondOpenConnectionReply {
    const ID: u8 = 0x08;
    fn packet_size(&self) -> usize {
        1 + 16 + 8 + 7 + 2 + 1 // packet id + magic + server guid + client address + mtu length + encryption
    }
}
