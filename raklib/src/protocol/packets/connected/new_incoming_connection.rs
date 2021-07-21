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
    fn decode(bstream: &mut raklib_std::utils::BinaryStream) -> Self
    where
        Self: Sized,
    {
        let server_address: SocketAddr = bstream.read();

        let sys_addresses: Vec<_> = (0..20).map(|_| bstream.read::<SocketAddr>()).collect();
        println!("System addreses: {:?}", sys_addresses);

        let ping_time: i64 = dbg!(bstream.read());
        let pong_time: i64 = dbg!(bstream.read());

        Self {
            server_address,
            internal_address: sys_addresses[0],
        }
    }
}
