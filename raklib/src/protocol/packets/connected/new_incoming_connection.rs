use std::net::SocketAddr;

use raklib_std::packet::{Packet, PacketDecode};

#[derive(Debug)]
pub struct NewIncomingConnection {
    pub server_address: SocketAddr,
    pub internal_address: SocketAddr,
}

impl Packet for NewIncomingConnection {
    const ID: u8 = 0x13;
}

impl PacketDecode for NewIncomingConnection {
    fn decode(bstream: &mut raklib_std::utils::BinaryStream) -> Self
    where
        Self: Sized,
    {
        crate::server::Server::print_binary(&bstream.data[..]);
        unimplemented!()
    }
}
