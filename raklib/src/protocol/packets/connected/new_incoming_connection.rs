use crate::*;
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

//FIXME: weird packet
impl PacketDecode for NewIncomingConnection {
    fn decode(bstream: &mut raklib_std::stream::BinaryStream) -> Self
    where
        Self: Sized,
    {
        let server_address: SocketAddr = bstream.read();

        let sys_addresses: Vec<_> = (0..20).map(|_| bstream.read::<SocketAddr>()).collect();
        debug!("System addresses: {:?}", sys_addresses);

        let _ping_time: i64 = bstream.read();
        let _pong_time: i64 = bstream.read();

        Self {
            server_address,
            internal_address: sys_addresses[0],
        }
    }
}
