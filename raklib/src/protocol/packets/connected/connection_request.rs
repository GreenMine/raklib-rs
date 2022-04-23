use std::{
    net::{SocketAddr, SocketAddrV4},
    str::FromStr,
};

use raklib_std::packet::{Packet, PacketEncode};

#[derive(raklib_derive::PacketDecode, Debug)]
pub struct ConnectionRequest {
    pub guid: u64,
    pub time: i64,
    pub use_security: bool,
}

impl Packet for ConnectionRequest {
    const ID: u8 = 0x09;
}

pub struct ConnectionRequestAccepted {
    pub client_address: SocketAddr,
    pub request_time: i64,
    pub time: i64,
}

impl ConnectionRequestAccepted {
    pub fn new(client_address: SocketAddr, request_time: i64, time: i64) -> Self {
        Self {
            client_address,
            request_time,
            time,
        }
    }
}

impl Packet for ConnectionRequestAccepted {
    const ID: u8 = 0x10;

    fn packet_size(&self) -> usize
    where
        Self: Sized,
    {
        1 + 7 + 2 + (20 * 7) + 8 + 8
    }
}

impl PacketEncode for ConnectionRequestAccepted {
    fn encode_payload(&self, bstream: &mut raklib_std::stream::BinaryStream) {
        bstream.add(self.client_address);
        bstream.add(0u16);

        std::iter::repeat(SocketAddr::V4(
            SocketAddrV4::from_str("255.255.255.255:19132").unwrap(),
        ))
        .take(20)
        .for_each(|e| {
            bstream.add(e);
        });

        bstream.add(self.request_time);
        bstream.add(self.time);
    }
}
