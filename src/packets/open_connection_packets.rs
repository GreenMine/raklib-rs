use std::net::SocketAddr;
use super::{PacketDecode, PacketEncode};
use crate::{types::Magic, utils::BinaryStream, consts};
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
    fn encode(&self) -> BinaryStream {
        let mut bstream = BinaryStream::with_len(1 + 16 + 8 + 1 + 2);

        bstream.add(0x06_u8);
        bstream.add_magic(consts::MAGIC);
        bstream.add(consts::SERVER_GUID);
        bstream.add(self.use_security);
        bstream.add(self.mtu_length);

        bstream
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
    fn decode(bstream: &mut BinaryStream) -> Self {
        Self {
            magic: bstream.read_magic(),
            server_address: bstream.read_address(),
            mtu_length: bstream.read(),
            client_guid: bstream.read()
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
    fn encode(&self) -> BinaryStream {
        let mut bstream = BinaryStream::with_len(1 + 16 + 8 + 7 + 2 + 1);

        bstream.add(0x08_u8);
        bstream.add_magic(consts::MAGIC);
        bstream.add(consts::SERVER_GUID);
        bstream.add_address(self.client_address);
        bstream.add(self.mtu_length);
        bstream.add(self.enctyption);

        bstream
    }
}