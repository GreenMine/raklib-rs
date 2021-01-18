use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, unimplemented};

use crate::{types::{Magic, RakNetString}, utils::BinaryStream};


pub struct Packet<'a> {
    pub id: u8,
    pub stream: &'a mut BinaryStream
}

impl<'a> Packet<'a> {
    pub fn new(packet_id: u8, packet_len: usize, bstream: &'a mut BinaryStream) -> Self {
        bstream.add(packet_id);

        Self { id: packet_id, stream: bstream }
    }

    pub fn from_stream(bstream: &'a mut BinaryStream) -> Self {
        Self {
            id: bstream.read(),
            stream: bstream
        }
    }

}

pub trait PacketDecode {
    fn decode(bstream: &mut BinaryStream) -> Self;
}

pub trait PacketEncode {
    fn encode(&self) -> BinaryStream;
}